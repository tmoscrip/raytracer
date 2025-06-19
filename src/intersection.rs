use crate::{
    ray::Ray,
    shape::Shape,
    tuple::{reflect, Tuple},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub object_id: u32,
}

impl Intersection {
    pub fn new(t: f64, object: &dyn Shape) -> Self {
        Intersection {
            t,
            object_id: object.data().id,
        }
    }
}

pub fn hit(xs: &[Intersection]) -> Option<&Intersection> {
    xs.iter()
        .filter(|intersection| intersection.t >= 0.0)
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal))
}

pub struct PreComputedData<'a> {
    pub t: f64,
    pub object: &'a dyn Shape,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub reflectv: Tuple,
    pub inside: bool,
    pub n1: f64,
    pub n2: f64,
}

fn intersection_eq(a: &Intersection, b: &Intersection) -> bool {
    a.object_id == b.object_id && (a.t - b.t).abs() < 1e-8
}

pub fn prepare_computations<'a>(
    hit: &Intersection,
    ray: &Ray,
    registry: &'a crate::shape_registry::ShapeRegistry,
    all_intersections: Option<&Vec<Intersection>>,
) -> Option<PreComputedData<'a>> {
    let sphere = registry.get(hit.object_id)?;
    let point = ray.position(hit.t);
    let eyev = -(ray.direction);
    let mut normalv = sphere.normal_at(&point);

    let inside: bool;
    if normalv.clone().dot(&eyev) < 0.0 {
        inside = true;
        normalv = -normalv;
    } else {
        inside = false;
    }

    let reflectv = reflect(&ray.direction, &normalv);

    let mut n1 = 1.0;
    let mut n2 = 1.0;
    let mut containers: Vec<&dyn Shape> = Vec::new();
    if let Some(all_intersections) = all_intersections {
        for i in all_intersections {
            println!(
                "t: {}, object_id: {}, containers: {:?}",
                i.t,
                i.object_id,
                containers
                    .iter()
                    .map(|o| (o.data().id, o.data().material.refractive_index))
                    .collect::<Vec<_>>()
            );
            // Set n1 before updating containers
            if intersection_eq(i, hit) {
                n1 = containers
                    .last()
                    .map_or(1.0, |obj| obj.data().material.refractive_index);
            }

            // Update containers
            let current_object = registry.get(i.object_id).unwrap();
            if let Some(pos) = containers
                .iter()
                .position(|&obj| obj.data().id == current_object.data().id)
            {
                containers.remove(pos);
            } else {
                containers.push(current_object);
            }

            // Set n2 after updating containers, then break
            if intersection_eq(i, hit) {
                n2 = containers
                    .last()
                    .map_or(1.0, |obj| obj.data().material.refractive_index);
                break;
            }
        }
    }

    Some(PreComputedData {
        t: hit.t,
        object: sphere,
        point: point.clone(),
        // Epsilon is too small, resulted in artifacts. Making it 50000 times larger works.
        over_point: point + normalv * 50000.0 * f64::EPSILON,
        eyev,
        normalv,
        reflectv,
        inside,
        n1,
        n2,
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        matrix::Matrix,
        shape::{plane::Plane, sphere::Sphere},
    };

    use super::*;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object_id, s.data.id);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2];

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn intersect_sets_object_on_intersection() {
        use crate::ray::Ray;
        use crate::tuple::Tuple;

        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object_id, s.data.id);
        assert_eq!(xs[1].object_id, s.data.id);
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i2.clone(), i1.clone()];
        let i = hit(&xs);

        assert_eq!(i, Some(&i1));
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = vec![i2.clone(), i1.clone()];
        let i = hit(&xs);

        assert_eq!(i, Some(&i2));
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = vec![i2.clone(), i1.clone()];
        let i = hit(&xs);

        assert_eq!(i, None);
    }

    #[test]
    fn hit_is_always_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()];
        let i = hit(&xs);

        assert_eq!(i, Some(&i4));
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let r = crate::ray::Ray::new(
            crate::tuple::Tuple::point(0.0, 0.0, -5.0),
            crate::tuple::Tuple::vector(0.0, 0.0, 1.0),
        );
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);

        // Create a registry and register the sphere
        let mut registry = crate::shape_registry::ShapeRegistry::new();
        registry.register(shape);

        let comps = prepare_computations(&i, &r, &registry, None).unwrap();

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object.id(), i.object_id);
        assert_eq!(comps.point, crate::tuple::Tuple::point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, crate::tuple::Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, crate::tuple::Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_when_intersection_occurs_on_outside() {
        let r = crate::ray::Ray::new(
            crate::tuple::Tuple::point(0.0, 0.0, -5.0),
            crate::tuple::Tuple::vector(0.0, 0.0, 1.0),
        );
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);

        // Create a registry and register the sphere
        let mut registry = crate::shape_registry::ShapeRegistry::new();
        registry.register(shape);

        let comps = prepare_computations(&i, &r, &registry, None).unwrap();

        assert_eq!(comps.inside, false);
    }

    #[test]
    fn hit_when_intersection_occurs_on_inside() {
        let r = crate::ray::Ray::new(
            crate::tuple::Tuple::point(0.0, 0.0, 0.0),
            crate::tuple::Tuple::vector(0.0, 0.0, 1.0),
        );
        let shape = Sphere::new();
        let i = Intersection::new(1.0, &shape);

        // Create a registry and register the sphere
        let mut registry = crate::shape_registry::ShapeRegistry::new();
        registry.register(shape);

        let comps = prepare_computations(&i, &r, &registry, None).unwrap();

        assert_eq!(comps.point, crate::tuple::Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, crate::tuple::Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
        // normal would have been (0, 0, 1), but is inverted!
        assert_eq!(comps.normalv, crate::tuple::Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_should_offset_the_point() {
        let r = crate::ray::Ray::new(
            crate::tuple::Tuple::point(0.0, 0.0, -5.0),
            crate::tuple::Tuple::vector(0.0, 0.0, 1.0),
        );
        let mut shape = Sphere::new();
        shape.set_transform(crate::matrix::Matrix::translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, &shape);

        let mut registry = crate::shape_registry::ShapeRegistry::new();
        registry.register(shape);

        let comps = prepare_computations(&i, &r, &registry, None).unwrap();

        assert!(comps.over_point.z < -f64::EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn precomputing_reflection_vector() {
        let plane = Plane::new();
        let r = Ray::new(
            Tuple::point(0.0, 1.0, -1.0),
            Tuple::vector(0.0, -(2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0),
        );
        let i = Intersection::new((2.0 as f64).sqrt(), &plane);

        let mut registry = crate::shape_registry::ShapeRegistry::new();
        registry.register(plane);

        let comps = prepare_computations(&i, &r, &registry, None).unwrap();
        assert_eq!(
            comps.reflectv,
            Tuple::vector(0.0, (2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0)
        )
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let mut registry = crate::shape_registry::ShapeRegistry::new();

        // A ← glass_sphere() with transform: scaling(2,2,2), refractive_index: 1.5
        let mut a = Sphere::glass();
        a.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        a.data_mut().material.refractive_index = 1.5;
        let a_id = registry.register(a);

        // B ← glass_sphere() with transform: translation(0,0,-0.25), refractive_index: 2.0
        let mut b = Sphere::glass();
        b.set_transform(Matrix::translation(0.0, 0.0, -0.25));
        b.data_mut().material.refractive_index = 2.0;
        let b_id = registry.register(b);

        // C ← glass_sphere() with transform: translation(0,0,0.25), refractive_index: 2.5
        let mut c = Sphere::glass();
        c.set_transform(Matrix::translation(0.0, 0.0, 0.25));
        c.data_mut().material.refractive_index = 2.5;
        let c_id = registry.register(c);

        // Get registered objects (with correct ids)
        let a = registry.get(a_id).unwrap();
        let b = registry.get(b_id).unwrap();
        let c = registry.get(c_id).unwrap();

        // r ← ray(point(0,0,-4), vector(0,0,1))
        let r = Ray::new(Tuple::point(0.0, 0.0, -4.0), Tuple::vector(0.0, 0.0, 1.0));

        // xs ← intersections(2:A, 2.75:B, 3.25:C, 4.75:B, 5.25:C, 6:A)
        let xs = vec![
            Intersection::new(2.0, a),
            Intersection::new(2.75, b),
            Intersection::new(3.25, c),
            Intersection::new(4.75, b),
            Intersection::new(5.25, c),
            Intersection::new(6.0, a),
        ];

        // Table of (index, expected_n1, expected_n2)
        let test_cases = vec![
            (0, 1.0, 1.5),
            (1, 1.5, 2.0),
            (2, 2.0, 2.5),
            (3, 2.5, 2.5),
            (4, 2.5, 1.5),
            (5, 1.5, 1.0),
        ];

        for (index, expected_n1, expected_n2) in test_cases {
            let comps =
                crate::intersection::prepare_computations(&xs[index], &r, &registry, Some(&xs))
                    .unwrap();
            assert_eq!(comps.n1, expected_n1, "Failed at index {}: n1", index);
            assert_eq!(comps.n2, expected_n2, "Failed at index {}: n2", index);
        }
    }
}

use crate::{
    ray::Ray,
    shape::{normal_at, Sphere},
    tuple::Tuple,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub sphere_id: u32,
}

impl Intersection {
    pub fn new(t: f64, sphere: &Sphere) -> Self {
        Intersection {
            t,
            sphere_id: sphere.id,
        }
    }
}

pub fn hit(xs: &[Intersection]) -> Option<&Intersection> {
    xs.iter()
        .filter(|intersection| intersection.t >= 0.0)
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal))
}

pub struct PreComputedData {
    pub t: f64,
    pub object: Sphere,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

pub fn prepare_computations(
    intersection: &Intersection,
    ray: &Ray,
    registry: &crate::sphere_registry::SphereRegistry,
) -> Option<PreComputedData> {
    let sphere = registry.get(intersection.sphere_id)?;
    let point = ray.position(intersection.t);
    let eyev = -(ray.direction);
    let mut normalv = normal_at(sphere, &point);

    let inside: bool;
    if normalv.clone().dot(&eyev) < 0.0 {
        inside = true;
        normalv = -normalv;
    } else {
        inside = false;
    }

    Some(PreComputedData {
        t: intersection.t,
        object: sphere.clone(),
        point: point.clone(),
        // Epsilon is too small, resulted in artifacts. Making it 50000 times larger works.
        over_point: point + normalv * 50000.0 * f64::EPSILON,
        eyev,
        normalv,
        inside,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::Sphere;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.sphere_id, s.id);
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
        use crate::shape::intersect;
        use crate::tuple::Tuple;

        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersect(&s, &r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].sphere_id, s.id);
        assert_eq!(xs[1].sphere_id, s.id);
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
        let mut registry = crate::sphere_registry::SphereRegistry::new();
        registry.register(shape);

        let comps = prepare_computations(&i, &r, &registry).unwrap();

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object.id, i.sphere_id);
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
        let mut registry = crate::sphere_registry::SphereRegistry::new();
        registry.register(shape);

        let comps = prepare_computations(&i, &r, &registry).unwrap();

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
        let mut registry = crate::sphere_registry::SphereRegistry::new();
        registry.register(shape);

        let comps = prepare_computations(&i, &r, &registry).unwrap();

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

        let mut registry = crate::sphere_registry::SphereRegistry::new();
        registry.register(shape);

        let comps = prepare_computations(&i, &r, &registry).unwrap();

        assert!(comps.over_point.z < -f64::EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}

use crate::sphere::Sphere;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::Sphere;

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
        use crate::sphere::intersect;
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
}

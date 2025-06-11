use crate::{
    intersection::{self, Intersection},
    materials::Material,
    matrix::Matrix,
    ray::Ray,
    shape::{Shape, ShapeData},
    tuple::Tuple,
};

#[derive(Clone)]
pub struct Plane {
    pub data: ShapeData,
}

impl Plane {
    pub fn new() -> Plane {
        let identity = Matrix::identity();
        Plane {
            data: ShapeData {
                id: 0,
                transform: identity.clone(),
                inverse_transform: identity.inverse(),
                material: Material::new(),
            },
        }
    }
}

impl Shape for Plane {
    fn data(&self) -> &ShapeData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut ShapeData {
        &mut self.data
    }

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        if ray.direction.y.abs() < f64::EPSILON * 50000.0 {
            return vec![];
        }

        let t = -ray.origin.y / ray.direction.y;
        return vec![Intersection::new(t, self)];
    }

    fn local_normal_at(&self, _local_point: &Tuple) -> Tuple {
        Tuple::vector(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn normal_of_plane_is_constant_everywhere() {
        let p = Plane::new();
        let n1 = p.local_normal_at(&Tuple::point(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(&Tuple::point(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(&Tuple::point(-5.0, 0.0, 150.0));

        assert_eq!(n1, Tuple::vector(0.0, 1.0, 0.0));
        assert_eq!(n2, Tuple::vector(0.0, 1.0, 0.0));
        assert_eq!(n3, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0.0, 10.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = p.local_intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = p.local_intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_abs_diff_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object_id, p.data.id);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0.0, -1.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_abs_diff_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object_id, p.data.id);
    }
}

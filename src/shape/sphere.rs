use crate::{
    intersection::Intersection,
    materials::Material,
    matrix::Matrix,
    ray::Ray,
    shape::{Shape, ShapeData},
    tuple::Tuple,
};

#[derive(Clone)]
pub struct Sphere {
    pub data: ShapeData,
}

impl Sphere {
    pub fn new() -> Sphere {
        let identity = Matrix::identity();
        Sphere {
            data: ShapeData {
                id: 0, // Temporary, will be set by registry
                transform: identity.clone(),
                inverse_transform: identity.inverse(),
                material: Material::new(),
            },
        }
    }

    pub fn glass() -> Sphere {
        let identity = Matrix::identity();
        let mut m = Material::new();
        m.transparency = 1.0;
        m.refractive_index = 1.5;
        Sphere {
            data: ShapeData {
                id: 0, // Temporary, will be set by registry
                transform: identity.clone(),
                inverse_transform: identity.inverse(),
                material: m,
            },
        }
    }
}

impl Shape for Sphere {
    fn data(&self) -> &ShapeData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut ShapeData {
        &mut self.data
    }

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return vec![];
        } else {
            let sqrt_discriminant = discriminant.sqrt();
            let inv_2a = 1.0 / (2.0 * a);
            let t1 = (-b - sqrt_discriminant) * inv_2a;
            let t2 = (-b + sqrt_discriminant) * inv_2a;

            vec![Intersection::new(t1, self), Intersection::new(t2, self)]
        }
    }

    fn local_normal_at(&self, local_point: &Tuple) -> Tuple {
        local_point.clone() - Tuple::point(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;
    use crate::ray::Ray;
    use crate::tuple::Tuple;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object_id, s.data.id);
        assert_eq!(xs[1].object_id, s.data.id);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(Matrix::translation(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn normal_on_sphere_at_point_on_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(&Tuple::point(1.0, 0.0, 0.0));

        assert_eq!(n, Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_point_on_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(&Tuple::point(0.0, 1.0, 0.0));

        assert_eq!(n, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_point_on_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(&Tuple::point(0.0, 0.0, 1.0));

        assert_eq!(n, Tuple::vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_on_sphere_at_nonaxial_point() {
        let s = Sphere::new();
        let sqrt_3_div_3 = (3.0_f64).sqrt() / 3.0;
        let n = s.normal_at(&Tuple::point(sqrt_3_div_3, sqrt_3_div_3, sqrt_3_div_3));

        assert_eq!(n, Tuple::vector(sqrt_3_div_3, sqrt_3_div_3, sqrt_3_div_3));
    }

    #[test]
    fn normal_is_normalized_vector() {
        let s = Sphere::new();
        let sqrt_3_div_3 = (3.0_f64).sqrt() / 3.0;
        let n = s.normal_at(&Tuple::point(sqrt_3_div_3, sqrt_3_div_3, sqrt_3_div_3));

        assert_eq!(n, n.normalise());
    }

    #[test]
    fn normal_on_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transform(Matrix::translation(0.0, 1.0, 0.0));
        let n = s.normal_at(&Tuple::point(0.0, 1.70711, -0.70711));

        assert_abs_diff_eq!(n, Tuple::vector(0.0, 0.70711, -0.70711), epsilon = 0.0001);
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let mut s = Sphere::new();
        let m = Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(std::f64::consts::PI / 5.0);
        s.set_transform(m);
        let sqrt_2_div_2 = (2.0_f64).sqrt() / 2.0;
        let n = s.normal_at(&Tuple::point(0.0, sqrt_2_div_2, -sqrt_2_div_2));

        assert_abs_diff_eq!(n, Tuple::vector(0.0, 0.97014, -0.24254), epsilon = 0.0001);
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Sphere::new();
        let m = s.data.material;
        let default = Material::new();

        assert_eq!(m.colour, default.colour);
        assert_eq!(m.ambient, default.ambient);
        assert_eq!(m.diffuse, default.diffuse);
        assert_eq!(m.specular, default.specular);
        assert_eq!(m.shininess, default.shininess);
    }

    #[test]
    fn sphere_may_be_assigned_material() {
        let mut s = Sphere::new();
        let mut m = crate::materials::Material::new();
        m.ambient = 1.0;
        s.set_material(m);

        assert_eq!(s.material().ambient, 1.0);
    }

    #[test]
    fn glassy_sphere_has_expected_properties() {
        let s = Sphere::glass();
        assert_eq!(s.material().transparency, 1.0);
        assert_eq!(s.material().refractive_index, 1.5);
    }
}

use crate::materials::Material;
use crate::matrix::Matrix;
use crate::tuple::Tuple;
use crate::{intersection::Intersection, ray::Ray};
use std::sync::atomic::{AtomicU32, Ordering};

static SPHERE_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

pub fn reset_sphere_counter() {
    SPHERE_ID_COUNTER.store(0, Ordering::Relaxed);
}

#[derive(Clone)]
pub struct ShapeData {
    pub id: u32,
    pub transform: Matrix,
    pub inverse_transform: Matrix,
    pub material: Material,
    // Optionally, add saved_ray for testing
    // pub saved_ray: Option<Ray>,
}

pub trait Shape {
    fn data(&self) -> &ShapeData;

    fn id(&self) -> u32;
    fn transform(&self) -> &Matrix;
    fn inverse_transform(&self) -> &Matrix;
    fn set_transform(&mut self, transform: Matrix);
    fn material(&self) -> &Material;
    fn set_material(&mut self, material: Material);

    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let local_ray = ray.clone().transform(&self.data().inverse_transform);
        // self.data_mut().saved_ray = Some(local_ray.clone()); // for testing
        self.local_intersect(&local_ray)
    }

    fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let object_point = self.data().inverse_transform.clone() * world_point.clone();
        let object_normal = self.local_normal_at(&object_point);
        let world_normal = self.data().inverse_transform.transpose() * object_normal;
        Tuple::vector(world_normal.x, world_normal.y, world_normal.z).normalise()
    }

    // Abstract methods
    fn local_intersect(&self, local_ray: &Ray) -> Vec<Intersection>;
    fn local_normal_at(&self, local_point: &Tuple) -> Tuple;
}

#[derive(Clone)]
pub struct Sphere {
    pub data: ShapeData,
}

impl Sphere {
    pub fn new() -> Sphere {
        let identity = Matrix::identity();
        Sphere {
            data: ShapeData {
                id: SPHERE_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
                transform: identity.clone(),
                inverse_transform: identity.inverse(),
                material: Material::new(),
            },
        }
    }
}

impl Shape for Sphere {
    fn id(&self) -> u32 {
        self.data.id
    }

    fn data(&self) -> &ShapeData {
        &self.data
    }

    fn transform(&self) -> &Matrix {
        &self.data.transform
    }

    fn inverse_transform(&self) -> &Matrix {
        &self.data.inverse_transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.data.inverse_transform = transform.inverse();
        self.data.transform = transform;
    }

    fn material(&self) -> &Material {
        &self.data.material
    }

    fn set_material(&mut self, material: Material) {
        self.data.material = material;
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
        assert_eq!(xs[0].sphere_id, s.data.id);
        assert_eq!(xs[1].sphere_id, s.data.id);
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
}

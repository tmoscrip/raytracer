use crate::materials::Material;
use crate::matrix::Matrix;
use crate::tuple::Tuple;
use crate::{intersection::Intersection, ray::Ray};

#[derive(Clone)]
pub struct ShapeData {
    pub id: u32,
    pub transform: Matrix,
    pub inverse_transform: Matrix,
    pub material: Material,
    // Optionally, add saved_ray for testing
    // pub saved_ray: Option<Ray>,
}

impl ShapeData {
    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}

pub trait Shape {
    fn id(&self) -> u32 {
        self.data().id
    }

    fn transform(&self) -> &Matrix {
        &self.data().transform
    }

    fn inverse_transform(&self) -> &Matrix {
        &self.data().inverse_transform
    }

    fn material(&self) -> &Material {
        &self.data().material
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.data_mut().inverse_transform = transform.inverse();
        self.data_mut().transform = transform;
    }

    fn set_material(&mut self, material: Material) {
        self.data_mut().material = material;
    }

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
    fn data(&self) -> &ShapeData;
    fn data_mut(&mut self) -> &mut ShapeData;
    fn local_intersect(&self, local_ray: &Ray) -> Vec<Intersection>;
    fn local_normal_at(&self, local_point: &Tuple) -> Tuple;
}

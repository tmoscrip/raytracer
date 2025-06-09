use crate::{
    colour::Colour,
    intersection::{hit, prepare_computations, Intersection, PreComputedData},
    light::Light,
    materials::lighting,
    ray::Ray,
    sphere::{intersect, Sphere},
    sphere_registry::SphereRegistry,
};

pub struct World {
    pub registry: SphereRegistry,
    pub light: Option<Light>,
}

impl World {
    pub fn new() -> Self {
        World {
            registry: SphereRegistry::new(),
            light: Option::None,
        }
    }

    pub fn add_object(&mut self, sphere: Sphere) -> u32 {
        self.registry.register(sphere)
    }

    pub fn default_world() -> Self {
        use crate::{colour::Colour, matrix::Matrix, tuple::Tuple};

        // Create default light
        let light_position = Tuple::point(-10.0, 10.0, -10.0);
        let light_intensity = Colour::new(1.0, 1.0, 1.0);
        let light = Light::point_light(light_position, light_intensity);

        // Create first sphere (s1)
        let mut s1 = Sphere::new();
        s1.material.colour = Colour::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        // Create second sphere (s2)
        let mut s2 = Sphere::new();
        s2.set_transform(Matrix::scaling(0.5, 0.5, 0.5));

        let mut world = World {
            registry: SphereRegistry::new(),
            light: Some(light),
        };

        world.add_object(s1);
        world.add_object(s2);

        world
    }

    pub fn intersect_world(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = Vec::new();
        for sphere in self.registry.iter() {
            let mut object_intersections = intersect(sphere, ray);
            intersections.append(&mut object_intersections);
        }

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        intersections
    }

    pub fn shade_hit(&self, comps: &PreComputedData) -> Colour {
        match self.light.clone() {
            Some(light) => lighting(
                comps.object.material.clone(),
                light,
                comps.point.clone(),
                comps.eyev.clone(),
                comps.normalv.clone(),
            ),
            None => Colour::new(0.0, 0.0, 0.0), // No light = black
        }
    }

    pub fn colour_at(&self, ray: &Ray) -> Colour {
        let xs = self.intersect_world(ray);
        let hit = hit(&xs);
        match hit {
            Some(hit) => {
                let comp = prepare_computations(hit, ray, &self.registry);
                match comp {
                    Some(comp) => self.shade_hit(&comp),
                    None => Colour::black(),
                }
            }
            None => Colour::black(),
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::{colour::Colour, ray::Ray, tuple::Tuple};

    use super::*;

    #[test]
    fn created_world_has_no_objects_or_light() {
        let world = World::new();

        assert_eq!(world.registry.len(), 0);
        assert!(world.light.is_none());
    }

    #[test]
    fn default_world_has_light_and_two_spheres() {
        let world = World::default_world();

        // Check light
        assert!(world.light.is_some());
        let light = world.light.unwrap();
        assert_eq!(light.position, Tuple::point(-10.0, 10.0, -10.0));
        assert_eq!(light.intensity, Colour::new(1.0, 1.0, 1.0));

        // Check we have 2 spheres
        assert_eq!(world.registry.len(), 2);

        // Check first sphere (s1) - by insertion order
        let s1 = world.registry.get_by_index(0).unwrap();
        assert_eq!(s1.material.colour, Colour::new(0.8, 1.0, 0.6));
        assert_eq!(s1.material.diffuse, 0.7);
        assert_eq!(s1.material.specular, 0.2);

        // Check second sphere (s2) - by insertion order
        let s2 = world.registry.get_by_index(1).unwrap();
        assert_eq!(s2.transform, crate::matrix::Matrix::scaling(0.5, 0.5, 0.5));
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = World::default_world();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let xs = w.intersect_world(&r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::default_world();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = w.registry.get_by_index(0).unwrap(); // first object in w
        let i = crate::intersection::Intersection::new(4.0, shape);

        let comps = crate::intersection::prepare_computations(&i, &r, &w.registry).unwrap();
        let c = w.shade_hit(&comps);

        assert_abs_diff_eq!(c, Colour::new(0.38066, 0.47583, 0.2855), epsilon = 0.0001);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = World::default_world();
        w.light = Some(Light::point_light(
            Tuple::point(0.0, 0.25, 0.0),
            Colour::new(1.0, 1.0, 1.0),
        ));
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = w.registry.get_by_index(1).unwrap(); // second object in w
        let i = crate::intersection::Intersection::new(0.5, shape);

        let comps = crate::intersection::prepare_computations(&i, &r, &w.registry).unwrap();
        let c = w.shade_hit(&comps);

        assert_abs_diff_eq!(c, Colour::new(0.90498, 0.90498, 0.90498), epsilon = 0.0001);
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default_world();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));

        let c = w.colour_at(&r);

        assert_eq!(c, Colour::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default_world();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let c = w.colour_at(&r);

        assert_abs_diff_eq!(c, Colour::new(0.38066, 0.47583, 0.2855), epsilon = 0.0001);
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let mut w = World::new();
        w.light = Some(Light::point_light(
            Tuple::point(-10.0, 10.0, -10.0),
            Colour::new(1.0, 1.0, 1.0),
        ));

        // Create spheres with ambient = 1.0
        let mut s1 = Sphere::new();
        s1.material.colour = Colour::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        s1.material.ambient = 1.0;

        let mut s2 = Sphere::new();
        s2.set_transform(crate::matrix::Matrix::scaling(0.5, 0.5, 0.5));
        s2.material.ambient = 1.0;

        w.add_object(s1);
        w.add_object(s2);

        let r = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));
        let c = w.colour_at(&r);

        // The color should be the inner object's material color
        let inner_color = w.registry.get_by_index(1).unwrap().material.colour;
        assert_eq!(c, inner_color);
    }
}

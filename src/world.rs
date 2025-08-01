use crate::{
    colour::Colour,
    intersection::{hit, prepare_computations, Intersection, PreComputedData},
    light::Light,
    materials::lighting,
    pattern::{
        checkered::Checkered, gradient::Gradient, ring::Ring, striped::Striped, Pattern,
        PatternType,
    },
    ray::Ray,
    shape::{plane::Plane, sphere::Sphere, Shape},
    shape_registry::ShapeRegistry,
    tuple::Tuple,
};

pub(crate) const MAX_BOUNCES: i32 = 5;

pub struct World {
    pub registry: ShapeRegistry,
    pub light: Option<Light>,
}

impl World {
    pub fn new() -> Self {
        World {
            registry: ShapeRegistry::new(),
            light: Option::None,
        }
    }

    pub fn add_object<T: Shape + 'static>(&mut self, object: T) -> u32 {
        self.registry.register(object)
    }

    pub fn default_world() -> Self {
        use crate::{colour::Colour, materials::Material, matrix::Matrix, tuple::Tuple};

        // Create default light
        let light_position = Tuple::point(-10.0, 10.0, -10.0);
        let light_intensity = Colour::new(1.0, 1.0, 1.0);
        let light = Light::point_light(light_position, light_intensity);

        // Create first sphere (s1)
        let mut s1 = Sphere::new();
        let mut s1_material = Material::new();
        s1_material.set_colour(Colour::new(0.8, 1.0, 0.6));
        s1_material.diffuse = 0.7;
        s1_material.specular = 0.2;
        s1.set_material(s1_material);

        // Create second sphere (s2)
        let mut s2 = Sphere::new();
        s2.set_transform(Matrix::scaling(0.5, 0.5, 0.5));

        let mut world = World {
            registry: ShapeRegistry::new(),
            light: Some(light),
        };

        world.add_object(s1);
        world.add_object(s2);

        world
    }

    pub fn test_world() -> Self {
        use crate::{colour::Colour, materials::Material, matrix::Matrix, tuple::Tuple};
        use std::f64::consts::PI;

        // Create light source
        let light_position = Tuple::point(-10.0, 10.0, -10.0);
        let light_intensity = Colour::new(1.0, 1.0, 1.0);
        let light = Light::point_light(light_position, light_intensity);

        let mut world = World {
            registry: ShapeRegistry::new(),
            light: Some(light),
        };

        // 1. Floor - extremely flattened sphere with matte texture
        let mut floor = Sphere::new();
        floor.set_transform(Matrix::scaling(10.0, 0.01, 10.0));
        let mut floor_material = Material::new();
        floor_material.colour = Colour::new(1.0, 0.9, 0.9);
        floor_material.specular = 0.0;
        floor.set_material(floor_material);
        world.add_object(floor);

        // 2. Left wall
        let mut left_wall = Sphere::new();
        left_wall.set_transform(
            Matrix::translation(0.0, 0.0, 5.0)
                * Matrix::rotation_y(-PI / 4.0)
                * Matrix::rotation_x(PI / 2.0)
                * Matrix::scaling(10.0, 0.01, 10.0),
        );
        let mut left_wall_material = Material::new();
        left_wall_material.colour = Colour::new(1.0, 0.9, 0.9);
        left_wall_material.specular = 0.0;
        left_wall.set_material(left_wall_material);
        world.add_object(left_wall);

        // 3. Right wall
        let mut right_wall = Sphere::new();
        right_wall.set_transform(
            Matrix::translation(0.0, 0.0, 5.0)
                * Matrix::rotation_y(PI / 4.0)
                * Matrix::rotation_x(PI / 2.0)
                * Matrix::scaling(10.0, 0.01, 10.0),
        );
        let mut right_wall_material = Material::new();
        right_wall_material.colour = Colour::new(1.0, 0.9, 0.9);
        right_wall_material.specular = 0.0;
        right_wall.set_material(right_wall_material);
        world.add_object(right_wall);

        // 4. Middle sphere - large green sphere
        let mut middle = Sphere::new();
        middle.set_transform(Matrix::translation(-0.5, 1.0, 0.5));
        let mut middle_material = Material::new();
        middle_material.colour = Colour::new(0.1, 1.0, 0.5);
        middle_material.diffuse = 0.7;
        middle_material.specular = 0.3;
        middle.set_material(middle_material);
        world.add_object(middle);

        // 5. Right sphere - smaller green sphere
        let mut right = Sphere::new();
        right.set_transform(Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5));
        let mut right_material = Material::new();
        right_material.colour = Colour::new(0.5, 1.0, 0.1);
        right_material.diffuse = 0.7;
        right_material.specular = 0.3;
        right.set_material(right_material);
        world.add_object(right);

        // 6. Left sphere - smallest sphere
        let mut left = Sphere::new();
        left.set_transform(
            Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33),
        );
        let mut left_material = Material::new();
        left_material.colour = Colour::new(1.0, 0.8, 0.1);
        left_material.diffuse = 0.7;
        left_material.specular = 0.3;
        left.set_material(left_material);
        world.add_object(left);

        world
    }

    pub fn third_world() -> Self {
        use crate::{colour::Colour, materials::Material, matrix::Matrix, tuple::Tuple};
        use std::f64::consts::PI;

        // Create light source positioned above and to the left
        let light_position = Tuple::point(-10.0, 5.0, -10.0);
        let light_intensity = Colour::new(1.0, 1.0, 1.0);
        let light = Light::point_light(light_position, light_intensity);

        let mut world = World {
            registry: ShapeRegistry::new(),
            light: Some(light),
        };

        // 1. Floor - a plane at y=0 with a matte finish
        let mut floor = Plane::new();
        let mut floor_material = Material::new();
        floor_material.colour = Colour::new(1.0, 0.9, 0.9);
        floor_material.specular = 0.0; // Matte finish
        floor_material.reflective = 0.2;
        let mut pattern = Ring::new(Colour::new(0.8, 0.8, 0.8), Colour::new(0.2, 0.2, 0.2));
        let pattern_transform = Matrix::scaling(0.3, 0.3, 0.3) * Matrix::rotation_y(PI / 2.0);
        pattern.set_transform(pattern_transform);
        floor_material.set_pattern(Some(PatternType::Ring(pattern)));
        floor.set_material(floor_material);
        world.add_object(floor);

        // 2. Wall as backdrop - plane rotated π/2 around x-axis and translated in z
        let mut wall = Plane::new();
        wall.set_transform(Matrix::translation(0.0, 0.0, 5.0) * Matrix::rotation_x(PI / 2.0));
        let mut wall_material = Material::new();
        wall_material.colour = Colour::new(1.0, 0.9, 0.9);
        wall_material.specular = 0.0;
        let mut pattern = Gradient::new(Colour::new(1.0, 0.0, 0.0), Colour::new(0.0, 0.0, 1.0));
        let pattern_transform = Matrix::scaling(7.0, 7.0, 7.0) * Matrix::rotation_z(PI / 2.0);
        pattern.set_transform(pattern_transform);
        wall_material.set_pattern(Some(PatternType::Gradient(pattern)));
        wall.set_material(wall_material);
        world.add_object(wall);

        // 3. Large middle sphere sitting on the floor
        let mut middle = Sphere::new();
        middle.set_transform(Matrix::translation(-0.5, 1.0, 0.5));
        let mut middle_material = Material::new();
        middle_material.colour = Colour::new(0.1, 1.0, 0.5);
        middle_material.diffuse = 0.7;
        middle_material.specular = 0.3;
        middle_material.reflective = 0.2;
        let mut pattern = Striped::new(Colour::new(0.1, 0.3, 0.9), Colour::white());
        let pattern_transform = Matrix::scaling(0.2, 0.2, 0.2)
            * Matrix::rotation_y(PI / 6.0)
            * Matrix::rotation_z(PI / 3.0);
        pattern.set_transform(pattern_transform);
        middle_material.set_pattern(Some(PatternType::Striped(pattern)));
        middle.set_material(middle_material);
        world.add_object(middle);

        // 4. Right sphere - smaller sphere on the floor
        let mut right = Sphere::new();
        right.set_transform(Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5));
        let mut right_material = Material::new();
        right_material.colour = Colour::new(0.5, 1.0, 0.1);
        right_material.diffuse = 0.7;
        right_material.specular = 0.3;
        let mut pattern = Checkered::new(Colour::new(0.3, 0.7, 0.2), Colour::white());
        let pattern_transform = Matrix::scaling(0.3, 0.3, 0.3);
        pattern.set_transform(pattern_transform);
        right_material.set_pattern(Some(PatternType::Checkered(pattern)));
        right.set_material(right_material);
        world.add_object(right);

        // 5. Left sphere - smallest sphere on the floor
        let mut left = Sphere::new();
        left.set_transform(
            Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33),
        );
        let mut left_material = Material::new();
        left_material.colour = Colour::new(1.0, 0.8, 0.1);
        left_material.diffuse = 0.7;
        left_material.specular = 0.3;
        left_material.reflective = 0.5;
        left.set_material(left_material);
        world.add_object(left);

        // 6. Partially embedded sphere - sphere that intersects with the floor
        let mut embedded = Sphere::new();
        embedded
            .set_transform(Matrix::translation(1.0, -0.2, -1.0) * Matrix::scaling(0.6, 0.6, 0.6));
        let mut embedded_material = Material::new();
        embedded_material.colour = Colour::new(0.8, 0.2, 0.8);
        embedded_material.diffuse = 0.7;
        embedded_material.specular = 0.3;
        embedded.set_material(embedded_material);
        world.add_object(embedded);

        world
    }

    pub fn intersect_world(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = Vec::new();
        for sphere in self.registry.iter() {
            let mut object_intersections = sphere.intersect(ray);
            intersections.append(&mut object_intersections);
        }

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        intersections
    }

    pub fn shade_hit(&self, comps: &PreComputedData, bounces_remaining: i32) -> Colour {
        let shadowed = self.is_shadowed(comps.over_point);

        let surface = match self.light.clone() {
            Some(light) => lighting(
                comps.object.material().clone(),
                &Sphere::new(),
                light,
                comps.point.clone(),
                comps.eyev.clone(),
                comps.normalv.clone(),
                shadowed,
            ),
            None => Colour::new(0.0, 0.0, 0.0), // No light = black
        };

        let reflected = self.reflected_colour(comps, bounces_remaining);

        surface + reflected
    }

    pub fn colour_at(&self, ray: &Ray, bounces_remaining: i32) -> Colour {
        let xs = self.intersect_world(ray);
        let hit = hit(&xs);
        match hit {
            Some(hit) => {
                let comp = prepare_computations(hit, ray, &self.registry, Some(&xs));
                match comp {
                    Some(comp) => self.shade_hit(&comp, bounces_remaining),
                    None => Colour::black(),
                }
            }
            None => Colour::black(),
        }
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool {
        let v = self.light.as_ref().unwrap().position - point.clone();
        let distance = v.clone().magnitude();
        let direction = v.normalise();

        let r = Ray::new(point, direction);
        let xs = self.intersect_world(&r);

        let hit = hit(&xs);
        match hit {
            Some(hit) => hit.t < distance,
            None => false,
        }
    }

    pub fn reflected_colour(&self, comps: &PreComputedData, bounces_remaining: i32) -> Colour {
        if bounces_remaining <= 0 {
            return Colour::black();
        }

        if comps.object.material().reflective == 0.0 {
            return Colour::black();
        }

        let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
        let c = self.colour_at(&reflect_ray, bounces_remaining - 1);

        c * comps.object.material().reflective
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
        assert_eq!(s1.material().colour, Colour::new(0.8, 1.0, 0.6));
        assert_eq!(s1.material().diffuse, 0.7);
        assert_eq!(s1.material().specular, 0.2);

        // Check second sphere (s2) - by insertion order
        let s2 = world.registry.get_by_index(1).unwrap();
        assert_eq!(
            *s2.transform(),
            crate::matrix::Matrix::scaling(0.5, 0.5, 0.5)
        );
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
        let i = crate::intersection::Intersection {
            t: 4.0,
            object_id: shape.id(),
        };

        let comps = crate::intersection::prepare_computations(&i, &r, &w.registry, None).unwrap();
        let c = w.shade_hit(&comps, MAX_BOUNCES);

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
        let i = crate::intersection::Intersection {
            t: 0.5,
            object_id: shape.id(),
        };

        let comps = crate::intersection::prepare_computations(&i, &r, &w.registry, None).unwrap();
        let c = w.shade_hit(&comps, MAX_BOUNCES);

        assert_abs_diff_eq!(c, Colour::new(0.90498, 0.90498, 0.90498), epsilon = 0.0001);
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default_world();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));

        let c = w.colour_at(&r, MAX_BOUNCES);

        assert_eq!(c, Colour::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default_world();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let c = w.colour_at(&r, MAX_BOUNCES);

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
        let mut s1_material = crate::materials::Material::new();
        s1_material.colour = Colour::new(0.8, 1.0, 0.6);
        s1_material.diffuse = 0.7;
        s1_material.specular = 0.2;
        s1_material.ambient = 1.0;
        s1.set_material(s1_material);

        let mut s2 = Sphere::new();
        s2.set_transform(crate::matrix::Matrix::scaling(0.5, 0.5, 0.5));
        let mut s2_material = crate::materials::Material::new();
        s2_material.ambient = 1.0;
        s2.set_material(s2_material);

        w.add_object(s1);
        w.add_object(s2);

        let r = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));
        let c = w.colour_at(&r, MAX_BOUNCES);

        // The color should be the inner object's material color
        let inner_color = w.registry.get_by_index(1).unwrap().material().colour;
        assert_eq!(c, inner_color);
    }

    #[test]
    fn no_shadow_when_nothing_collinear_with_point_and_light() {
        let w = World::default_world();
        let p = Tuple::point(0.0, 10.0, 0.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn shadow_when_object_between_point_and_light() {
        let w = World::default_world();
        let p = Tuple::point(10.0, -10.0, 10.0);

        assert!(w.is_shadowed(p));
    }

    #[test]
    fn no_shadow_when_object_behind_light() {
        let w = World::default_world();
        let p = Tuple::point(-20.0, 20.0, -20.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn no_shadow_when_object_behind_point() {
        let w = World::default_world();
        let p = Tuple::point(-2.0, 2.0, -2.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = World::new();
        w.light = Some(Light::point_light(
            Tuple::point(0.0, 0.0, -10.0),
            Colour::new(1.0, 1.0, 1.0),
        ));

        let s1 = Sphere::new();
        w.add_object(s1);

        let mut s2 = Sphere::new();
        s2.set_transform(crate::matrix::Matrix::translation(0.0, 0.0, 10.0));
        let s2_id = w.add_object(s2);

        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let i = Intersection {
            t: 4.0,
            object_id: s2_id,
        };

        let comps = prepare_computations(&i, &r, &w.registry, None).unwrap();
        let c = w.shade_hit(&comps, MAX_BOUNCES);

        assert_eq!(c, Colour::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn reflected_colour_for_nonreflective_material() {
        let mut w = World::default_world();
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));

        // Get the second object (index 1) from the default world
        let shape_id = w.registry.get_by_index(1).unwrap().id();
        let shape = w.registry.get_mut(shape_id).unwrap();
        let mut mat = shape.material().clone();
        mat.ambient = 1.0;
        shape.set_material(mat);

        let i = Intersection::new(1.0, &*w.registry.get(shape_id).unwrap());
        let comps = prepare_computations(&i, &r, &w.registry, None).unwrap();
        let color = w.reflected_colour(&comps, MAX_BOUNCES);

        assert_eq!(color, Colour::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn reflected_colour_for_reflective_material() {
        let mut w = World::default_world();

        let mut shape = Plane::new();
        let mut mat = shape.material().clone();
        mat.reflective = 0.5;
        shape.set_material(mat);
        shape.set_transform(crate::matrix::Matrix::translation(0.0, -1.0, 0.0));
        let shape_id = w.add_object(shape);

        let r = Ray::new(
            Tuple::point(0.0, 0.0, -3.0),
            Tuple::vector(
                0.0,
                -std::f64::consts::SQRT_2 / 2.0,
                std::f64::consts::SQRT_2 / 2.0,
            ),
        );
        let i = Intersection::new(
            std::f64::consts::SQRT_2,
            &*w.registry.get(shape_id).unwrap(),
        );
        let comps = prepare_computations(&i, &r, &w.registry, None).unwrap();
        let colour = w.reflected_colour(&comps, MAX_BOUNCES);

        assert_abs_diff_eq!(colour.r, 0.19032, epsilon = 0.0001);
        assert_abs_diff_eq!(colour.g, 0.2379, epsilon = 0.0001);
        assert_abs_diff_eq!(colour.b, 0.14274, epsilon = 0.0001);
    }

    #[test]
    fn shade_hit_with_reflective_material() {
        let mut w = World::default_world();

        let mut shape = Plane::new();
        let mut mat = shape.material().clone();
        mat.reflective = 0.5;
        shape.set_material(mat);
        shape.set_transform(crate::matrix::Matrix::translation(0.0, -1.0, 0.0));
        let shape_id = w.add_object(shape);

        let r = Ray::new(
            Tuple::point(0.0, 0.0, -3.0),
            Tuple::vector(
                0.0,
                -std::f64::consts::SQRT_2 / 2.0,
                std::f64::consts::SQRT_2 / 2.0,
            ),
        );
        let i = Intersection::new(
            std::f64::consts::SQRT_2,
            &*w.registry.get(shape_id).unwrap(),
        );
        let comps = prepare_computations(&i, &r, &w.registry, None).unwrap();
        let colour = w.shade_hit(&comps, MAX_BOUNCES);

        assert_abs_diff_eq!(colour.r, 0.87677, epsilon = 0.0001);
        assert_abs_diff_eq!(colour.g, 0.92436, epsilon = 0.0001);
        assert_abs_diff_eq!(colour.b, 0.82918, epsilon = 0.0001);
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = World::new();
        w.light = Some(Light::point_light(
            Tuple::point(0.0, 0.0, 0.0),
            Colour::new(1.0, 1.0, 1.0),
        ));

        let mut lower = Plane::new();
        let mut lower_mat = lower.material().clone();
        lower_mat.reflective = 1.0;
        lower.set_material(lower_mat);
        lower.set_transform(crate::matrix::Matrix::translation(0.0, -1.0, 0.0));
        w.add_object(lower);

        let mut upper = Plane::new();
        let mut upper_mat = upper.material().clone();
        upper_mat.reflective = 1.0;
        upper.set_material(upper_mat);
        upper.set_transform(crate::matrix::Matrix::translation(0.0, 1.0, 0.0));
        w.add_object(upper);

        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));

        // This should terminate successfully without infinite recursion
        let _colour = w.colour_at(&r, MAX_BOUNCES);
    }

    #[test]
    fn reflected_color_at_maximum_recursive_depth() {
        let mut w = World::default_world();

        let mut shape = Plane::new();
        let mut material = shape.material().clone();
        material.reflective = 0.5;
        shape.set_material(material);
        shape.set_transform(crate::matrix::Matrix::translation(0.0, -1.0, 0.0));
        let shape_id = w.add_object(shape);

        let r = Ray::new(
            Tuple::point(0.0, 0.0, -3.0),
            Tuple::vector(
                0.0,
                -std::f64::consts::SQRT_2 / 2.0,
                std::f64::consts::SQRT_2 / 2.0,
            ),
        );
        let i = Intersection::new(
            std::f64::consts::SQRT_2,
            &*w.registry.get(shape_id).unwrap(),
        );
        let comps = prepare_computations(&i, &r, &w.registry, None).unwrap();

        let color = w.reflected_colour(&comps, 0);

        assert_eq!(color, Colour::black());
    }
}

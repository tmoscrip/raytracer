use crate::shape::Sphere;
use std::collections::HashMap;

pub struct SphereRegistry {
    spheres: HashMap<u32, Sphere>,
    insertion_order: Vec<u32>, // Track insertion order for indexing
}

impl SphereRegistry {
    pub fn new() -> Self {
        SphereRegistry {
            spheres: HashMap::new(),
            insertion_order: Vec::new(),
        }
    }

    pub fn register(&mut self, sphere: Sphere) -> u32 {
        let id = sphere.id;
        self.spheres.insert(id, sphere);
        self.insertion_order.push(id);
        id
    }

    pub fn get(&self, id: u32) -> Option<&Sphere> {
        self.spheres.get(&id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut Sphere> {
        self.spheres.get_mut(&id)
    }

    pub fn get_all_spheres(&self) -> Vec<&Sphere> {
        self.spheres.values().collect()
    }

    // Get sphere by insertion order (0-based indexing)
    pub fn get_by_index(&self, index: usize) -> Option<&Sphere> {
        self.insertion_order
            .get(index)
            .and_then(|id| self.spheres.get(id))
    }

    // Get sphere by insertion order (0-based indexing) - mutable
    pub fn get_by_index_mut(&mut self, index: usize) -> Option<&mut Sphere> {
        if let Some(&id) = self.insertion_order.get(index) {
            self.spheres.get_mut(&id)
        } else {
            None
        }
    }

    // Number of spheres in registry
    pub fn len(&self) -> usize {
        self.spheres.len()
    }

    pub fn is_empty(&self) -> bool {
        self.spheres.is_empty()
    }

    // Find sphere by predicate
    pub fn find_sphere<F>(&self, predicate: F) -> Option<&Sphere>
    where
        F: Fn(&Sphere) -> bool,
    {
        self.spheres.values().find(|sphere| predicate(sphere))
    }

    // Iterator over spheres in insertion order
    pub fn iter(&self) -> impl Iterator<Item = &Sphere> {
        self.insertion_order
            .iter()
            .filter_map(move |id| self.spheres.get(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_can_store_and_retrieve_sphere() {
        let mut registry = SphereRegistry::new();
        let sphere = Sphere::new();
        let id = sphere.id;

        registry.register(sphere);
        let retrieved = registry.get(id);

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, id);
    }

    #[test]
    fn registry_returns_none_for_nonexistent_id() {
        let registry = SphereRegistry::new();
        let result = registry.get(999);

        assert!(result.is_none());
    }

    #[test]
    fn registry_tracks_insertion_order() {
        let mut registry = SphereRegistry::new();
        let sphere1 = Sphere::new();
        let sphere2 = Sphere::new();
        let id1 = sphere1.id;
        let id2 = sphere2.id;

        registry.register(sphere1);
        registry.register(sphere2);

        assert_eq!(registry.len(), 2);
        assert_eq!(registry.get_by_index(0).unwrap().id, id1);
        assert_eq!(registry.get_by_index(1).unwrap().id, id2);
        assert!(registry.get_by_index(2).is_none());
    }

    #[test]
    fn registry_find_sphere_works() {
        let mut registry = SphereRegistry::new();
        let mut sphere = Sphere::new();
        sphere.material.ambient = 0.5;

        registry.register(sphere);

        let found = registry.find_sphere(|s| s.material.ambient == 0.5);
        assert!(found.is_some());

        let not_found = registry.find_sphere(|s| s.material.ambient == 0.9);
        assert!(not_found.is_none());
    }

    #[test]
    fn registry_iterator_works() {
        let mut registry = SphereRegistry::new();
        let sphere1 = Sphere::new();
        let sphere2 = Sphere::new();
        let id1 = sphere1.id;
        let id2 = sphere2.id;

        registry.register(sphere1);
        registry.register(sphere2);

        let ids: Vec<u32> = registry.iter().map(|s| s.id).collect();
        assert_eq!(ids, vec![id1, id2]);
    }
}

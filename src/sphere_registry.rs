use crate::shape::Shape;
use std::collections::HashMap;

pub struct ShapeRegistry {
    shapes: HashMap<u32, Box<dyn Shape>>,
    insertion_order: Vec<u32>, // Track insertion order for indexing
    next_id: u32,              // Counter for unique shape IDs
}

impl ShapeRegistry {
    pub fn new() -> Self {
        ShapeRegistry {
            shapes: HashMap::new(),
            insertion_order: Vec::new(),
            next_id: 0,
        }
    }

    pub fn register<T: Shape + 'static>(&mut self, mut object: T) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        object.data_mut().set_id(id);
        self.shapes.insert(id, Box::new(object));
        self.insertion_order.push(id);
        id
    }

    pub fn get(&self, id: u32) -> Option<&dyn Shape> {
        self.shapes.get(&id).map(|s| s.as_ref())
    }

    pub fn get_all_spheres(&self) -> Vec<&dyn Shape> {
        self.shapes.values().map(|s| s.as_ref()).collect()
    }

    // Get sphere by insertion order (0-based indexing)
    pub fn get_by_index(&self, index: usize) -> Option<&dyn Shape> {
        self.insertion_order
            .get(index)
            .and_then(|id| self.shapes.get(id))
            .map(|s| s.as_ref())
    }

    // Number of spheres in registry
    pub fn len(&self) -> usize {
        self.shapes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }

    // Find sphere by predicate
    pub fn find_sphere<F>(&self, predicate: F) -> Option<&dyn Shape>
    where
        F: Fn(&dyn Shape) -> bool,
    {
        self.shapes
            .values()
            .map(|s| s.as_ref())
            .find(|sphere| predicate(*sphere))
    }

    // Iterator over spheres in insertion order
    pub fn iter(&self) -> impl Iterator<Item = &dyn Shape> {
        self.insertion_order
            .iter()
            .filter_map(move |id| self.shapes.get(id))
            .map(|s| s.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::sphere::Sphere;

    #[test]
    fn registry_can_store_and_retrieve_sphere() {
        let mut registry = ShapeRegistry::new();
        let sphere = Sphere::new();
        let id = sphere.id();

        registry.register(sphere);
        let retrieved = registry.get(id);

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id(), id);
    }

    #[test]
    fn registry_returns_none_for_nonexistent_id() {
        let registry = ShapeRegistry::new();
        let result = registry.get(999);

        assert!(result.is_none());
    }
}

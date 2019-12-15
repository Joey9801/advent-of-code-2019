use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

struct Object {
    /// Index into the storage vector for the object that this object orbits
    ///
    /// COM doesn't orbit anything
    parent_id: Option<usize>,

    /// Cache of all the objcts that orbit this one - objects that have this one as their parent_id
    ///
    /// This field is kept up to date by the inherant impl methods on ObjectMap
    children: Vec<usize>,

    /// How many transitive+direct orbits does this object have. COM has a depth of 0.
    ///
    /// Starts out as None
    depth: Option<u32>,
}

struct OrbitMap {
    object_storage: Vec<Object>,

    /// Maps object name to index into object_storage
    object_names: HashMap<String, usize>,
}

impl OrbitMap {
    pub fn new() -> Self {
        OrbitMap {
            object_storage: Vec::new(),
            object_names: HashMap::new(),
        }
    }

    /// Gets the ID for the named object, or creates a new one.
    fn get_or_create_object(&mut self, object_name: &str) -> usize  {
        match self.object_names.get(object_name) {
            Some(id) => id.clone(),
            None => {
                let id = self.object_storage.len();

                self.object_storage.push(Object {
                    parent_id: None,
                    children: Vec::new(),
                    depth: None,
                });
                self.object_names.insert(object_name.to_string(), id);

                id
            }
        }
    }

    pub fn add_orbit(&mut self, orbit_str: &str) {
        let mut parts = orbit_str.trim().split(")");

        let parent_name = parts.next().expect("Invalid orbit definition");
        let parent_id = self.get_or_create_object(parent_name);

        let child_name = parts.next().expect("Invalid orbit definition");
        let child_id = self.get_or_create_object(child_name);

        if self.object_storage[child_id].parent_id.is_some() {
            panic!("Object '{}' has multiple parents");
        }

        self.object_storage[child_id].parent_id = Some(parent_id);
        self.object_storage[parent_id].children.push(child_id);
    }

    /// Fill in the depth field of every object
    pub fn compute_depths(&mut self) {
        let mut process_list: Vec<usize> = self.object_storage
            .iter()
            .enumerate()
            .filter(|(id, object)| object.parent_id.is_none())
            .map(|(id, object)| id)
            .collect();

        while process_list.len() > 0 {
            let id = process_list.pop().unwrap();
            let depth = match self.object_storage[id].parent_id {
                Some(parent_id) => self.object_storage[parent_id].depth.map(|d| d + 1),
                None => Some(0),
            };

            self.object_storage[id].depth = depth;
            process_list.extend(&self.object_storage[id].children);
        }
    }
}

fn main() {
    let file = File::open("./input.txt").expect("Failed to open input.txt");

    let mut orbit_map = OrbitMap::new();
    for line in BufReader::new(file).lines() {
        orbit_map.add_orbit(&line.expect("Failed to read line"));
    }

    orbit_map.compute_depths();

    let total: u32 = orbit_map.object_storage
        .iter()
        .filter_map(|o| o.depth)
        .sum();

    println!("Total orbit count: {:?}", total);
}
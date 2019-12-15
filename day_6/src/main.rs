use std::fs::File;
use std::io::{prelude::*, BufReader};
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
            .filter(|(_id, object)| object.parent_id.is_none())
            .map(|(id, _object)| id)
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

    /// The ID of the first common ancestor of two nodes
    pub fn lowest_common_ancestor(&self, a: usize, b: usize) -> Option<usize> {
        // Populate a set of A's lineage. For deep maps a HashSet would be more efficient.
        let mut a_ancestry = Vec::new();
        let mut cursor = Some(a);
        while cursor.is_some() {
            a_ancestry.push(cursor.unwrap());
            cursor = self.object_storage[cursor.unwrap()].parent_id;
        }

        // Find the first element in B's lineage that is in A's lineage.
        cursor = Some(b);
        while cursor.is_some() {
            if a_ancestry.contains(&cursor.unwrap()) {
                return cursor;
            }
            cursor = self.object_storage[cursor.unwrap()].parent_id;
        }

        None
    }
}

fn main() {
    let file = File::open("./input.txt").expect("Failed to open input.txt");

    let mut orbit_map = OrbitMap::new();
    for line in BufReader::new(file).lines() {
        orbit_map.add_orbit(&line.expect("Failed to read line"));
    }
    orbit_map.compute_depths();

    let you_id = *orbit_map.object_names.get("YOU").expect("There is no object called YOU");
    let san_id = *orbit_map.object_names.get("SAN").expect("There is no object called SAN");

    let source_id = orbit_map.object_storage[you_id].parent_id.expect("YOU is a root");
    let target_id = orbit_map.object_storage[san_id].parent_id.expect("SAN is a root");

    let lca_id = orbit_map.lowest_common_ancestor(source_id, target_id).expect("YOU and SAN share no common ancestor");

    let source_depth = orbit_map.object_storage[source_id].depth.unwrap();
    let target_depth = orbit_map.object_storage[target_id].depth.unwrap();
    let lca_depth = orbit_map.object_storage[lca_id].depth.unwrap();

    let path_len = source_depth + target_depth - 2*lca_depth;

    println!("Shortest path from YOU.parent -> SAN.parent = {}", path_len);
}
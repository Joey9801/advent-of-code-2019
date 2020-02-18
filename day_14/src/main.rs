use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::iter::FromIterator;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct CompoundId(usize);

struct CompoundBook {
    name_to_id_map: HashMap<String, CompoundId>,
    counter: CompoundId,
}

impl CompoundBook {
    fn new() -> Self {
        Self {
            name_to_id_map: HashMap::new(),
            counter: CompoundId(0),
        }
    }

    fn get(&self, name: &str) -> Option<CompoundId> {
        self.name_to_id_map.get(name).map(|id| *id)
    }

    fn get_or_add(&mut self, name: &str) -> CompoundId {
        if let Some(id) = self.name_to_id_map.get(name) {
            *id
        } else {
            let id = self.counter;
            self.counter.0 += 1;
            self.name_to_id_map.insert(name.to_string(), id);
            id
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct RecipeComponent {
    compound: CompoundId,
    quantity: u32,
}

#[derive(Debug)]
struct Recipe {
    inputs: Vec<RecipeComponent>,
    output: RecipeComponent,
}

impl Recipe {
    fn parse_from_str(s: &str, compounds: &mut CompoundBook) -> Self {
        let tokens = s.split_whitespace()
            .filter(|tok| *tok != "=>")
            .map(|tok| tok.trim_matches(','))
            .collect::<Vec<_>>();
        
        let mut components = tokens.chunks(2)
            .map(|chunk| RecipeComponent {
                quantity: chunk[0].parse().unwrap(),
                compound: compounds.get_or_add(chunk[1]),
            })
            .collect::<Vec<_>>();

        let output = components.pop().unwrap();
        let inputs = components;

        Self {
            inputs,
            output,
        }
    }
}

struct RecipeBook {
    compounds: CompoundBook,
    recipes: Vec<Recipe>,

    /// Maps a compound to the recipe that makes it
    output_map: HashMap<CompoundId, usize>,
}

impl RecipeBook {
    fn load_from_file(path: &std::path::Path) -> Self {
        let mut compounds = CompoundBook::new();

        // Ensure ORE/FUEL get id's 0/1
        assert_eq!(CompoundId(0), compounds.get_or_add("ORE"));
        assert_eq!(CompoundId(1), compounds.get_or_add("FUEL"));

        let f = File::open(path).expect("Failed to open recipe file");
        let reader = BufReader::new(f);
        let recipes = reader.lines()
            .map(|line| line.expect("Failed to read line of recipe file"))
            .map(|line| Recipe::parse_from_str(&line, &mut compounds))
            .collect::<Vec<_>>();

        let output_map = HashMap::from_iter(recipes.iter()
            .enumerate()
            .map(|(idx, recipe)| (recipe.output.compound, idx))
        );

        Self {
            compounds,
            recipes,
            output_map,
        }
    }

    fn get_for_output(&self, id: CompoundId) -> &Recipe {
        let recipe_idx = self.output_map
            .get(&id)
            .expect(&format!("Don't have reciped to make {:?}", id));
        
        &self.recipes[*recipe_idx]
    }
}


/// Calculates how much ORE is needed to make 1 FUEL
fn part_1(recipes: &RecipeBook) -> u32 {
    let mut needs = std::iter::repeat(0u32)
        .take(recipes.compounds.counter.0)
        .collect::<Vec<_>>();
    let mut leftovers = needs.clone();

    let ore_idx = 0usize;
    let fuel_idx = 1usize;

    needs[fuel_idx] = 1;

    let mut any_work_done = true;
    while any_work_done {
        any_work_done = false;
        for id in 1..needs.len() {
            if needs[id] == 0 {
                continue;
            }

            any_work_done = true;
            let recipe = recipes.get_for_output(CompoundId(id));

            // To satisfy the need for this compound, the recipe must be repeated `multiple` times
            let mut multiple = needs[id] / recipe.output.quantity;
            let leftover = (recipe.output.quantity - (needs[id] % recipe.output.quantity))
                 % recipe.output.quantity;
            if leftover != 0 {
                multiple += 1;
            }

            for input in &recipe.inputs {
                let id = input.compound.0;
                needs[id] += input.quantity * multiple;
                let leftover_to_use = std::cmp::min(needs[id], leftovers[id]);
                needs[id] -= leftover_to_use;
                leftovers[id] -= leftover_to_use;
            }

            needs[id] = 0;
            leftovers[id] += leftover;
        }
    }


    needs[ore_idx]
}

fn main() {
    let recipe_book = RecipeBook::load_from_file(Path::new("./input.txt"));
    
    // Sanity check that there is only one way to make each thing
    {
        let mut outputs = HashMap::new();
        for recipe in &recipe_book.recipes {
            if !outputs.contains_key(&recipe.output.compound) {
                outputs.insert(recipe.output.compound, 0);
            }
            *outputs.get_mut(&recipe.output.compound).unwrap() += 1;
        }
        if outputs.values().max() != Some(&1) {
            panic!("There are multiple ways to make some compounds");
        }
    }

    dbg!(part_1(&recipe_book));
}

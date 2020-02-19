use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::iter::FromIterator;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct CompoundId(usize);

/// Maps compound names to integer IDs.
///
/// Guarantees that issued IDs are in the range (0, CompoundBook::len()]
/// ORE and FUEL have static IDs of CompoundId(0) and CompoundId(1) respectively.
struct CompoundBook {
    name_to_id_map: HashMap<String, CompoundId>,
}

impl CompoundBook {
    fn new() -> Self {
        Self {
            name_to_id_map: HashMap::new(),
        }
    }

    fn len(&self) -> usize {
        self.name_to_id_map.len()
    }

    fn get_or_add(&mut self, name: &str) -> CompoundId {
        if let Some(id) = self.name_to_id_map.get(name) {
            *id
        } else {
            let id = CompoundId(self.name_to_id_map.len());
            self.name_to_id_map.insert(name.to_string(), id);
            id
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct RecipeComponent {
    compound: CompoundId,
    quantity: u64,
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


/// Calculates how much ORE is needed to make a given amount of FUEL
fn ore_for_fuel(recipes: &RecipeBook, required_fuel: u64) -> u64 {
    let mut needs = std::iter::repeat(0u64)
        .take(recipes.compounds.len())
        .collect::<Vec<_>>();
    let mut leftovers = needs.clone();

    let ore_idx = 0usize;
    let fuel_idx = 1usize;

    needs[fuel_idx] = required_fuel;

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

/// How much FUEL can be made from a given amount of ore
fn fuel_for_ore(recipes: &RecipeBook, given_ore: u64) -> u64 {
    // Just do a binary search on ore_for_fuel

    let mut low = 0u64;
    let mut high = None;

    while low + 1 < high.unwrap_or(u64::max_value()) {
        let test = match high {
            Some(high) => (low + high) / 2,
            None => (low * 2) + 1,
        };

        let ore_for_test = ore_for_fuel(recipes, test);

        match given_ore.cmp(&ore_for_test) {
            Ordering::Less => high = Some(test),
            Ordering::Greater => low = test,
            Ordering::Equal => return test,
        }
    }

    low
}

fn main() {
    let recipe_book = RecipeBook::load_from_file(Path::new("./input.txt"));
    
    // Sanity check that there is only one way to make each thing
    {
        let mut outputs = std::iter::repeat(0)
            .take(recipe_book.compounds.len())
            .collect::<Vec<_>>();
        for recipe in &recipe_book.recipes {
            outputs[recipe.output.compound.0] += 1;
        }
        if outputs.iter().max() != Some(&1) {
            panic!("There are multiple ways to make some compounds");
        }
    }

    dbg!(ore_for_fuel(&recipe_book, 1));
    dbg!(fuel_for_ore(&recipe_book, 1000_000_000_000));
}

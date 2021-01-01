#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl Food {
    fn new(line: String) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^(?P<ingredients>\w+( \w+)*) \(contains (?P<allergens>\w+(, \w+)*)\)$"
            )
            .unwrap();
        }
        let captures = RE.captures(&line).unwrap();
        let ingredients = captures
            .name("ingredients")
            .unwrap()
            .as_str()
            .split(" ")
            .map(str::to_string)
            .collect();
        let allergens = captures
            .name("allergens")
            .unwrap()
            .as_str()
            .split(", ")
            .map(str::to_string)
            .collect();
        Self {
            ingredients,
            allergens,
        }
    }
}

fn load_foods_from_file(filename: &str) -> Result<Vec<Food>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    Ok(lines.filter_map(Result::ok).map(Food::new).collect())
}

fn map_allergens(foods: &Vec<Food>) -> HashMap<String, String> {
    let mut possibilities: HashMap<String, Vec<String>> = HashMap::new();
    for food in foods {
        for allergen in &food.allergens {
            possibilities.insert(
                allergen.clone(),
                if let Some(poss) = possibilities.get(allergen) {
                    poss.iter()
                        .filter(|&a| food.ingredients.contains(a))
                        .cloned()
                        .collect()
                } else {
                    food.ingredients.clone()
                },
            );
        }
    }

    let mut allergen_map = HashMap::new();
    while possibilities.len() > 0 {
        // find allergens with only one possibility
        let known_allergens: Vec<_> = possibilities
            .iter()
            .filter_map(|(allergen, ingredients)| match ingredients.len() {
                1 => Some(allergen),
                _ => None,
            })
            .cloned()
            .collect();
        for allergen in known_allergens {
            let ingredient = possibilities.remove(&allergen).unwrap().pop().unwrap();
            // remove ingredient from remaining possibilities
            possibilities = possibilities
                .into_iter()
                .map(|(allergen, ingredients)| {
                    (
                        allergen,
                        ingredients
                            .into_iter()
                            .filter(|i| i != &ingredient)
                            .collect(),
                    )
                })
                .collect();
            allergen_map.insert(ingredient, allergen);
        }
    }
    allergen_map
}

fn count_inert_ingredients(foods: &Vec<Food>, allergens: &HashMap<String, String>) -> usize {
    foods
        .iter()
        .flat_map(|food| food.ingredients.iter())
        .filter(|&ingredient| allergens.get(ingredient).is_none())
        .count()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let foods = load_foods_from_file("inputs/day-21.txt")?;
    // println!("Foods: {:?}", foods);

    let allergens = map_allergens(&foods);
    // println!("Allergenic ingredients: {:?}", allergens);

    println!(
        "Part 1: Appearances of non-allergenic ingredients: {}",
        count_inert_ingredients(&foods, &allergens)
    );

    let mut dangerous_ingredients: Vec<_> = allergens.keys().cloned().collect();
    dangerous_ingredients.sort_unstable_by_key(|k| allergens.get(k).unwrap());
    println!(
        "Part 2: Canonical dangerous ingredient list: {}",
        dangerous_ingredients.join(",")
    );

    Ok(())
}

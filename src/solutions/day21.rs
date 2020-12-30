use std::collections::{HashMap, HashSet, BTreeMap};

pub fn run(input: &str) {
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> u64 {
    let food_and_allergens = parse_input(input);
    let all_ingredients = food_and_allergens.iter()
        .map(|(ingredients, _allergens)| ingredients)
        .flatten()
        .collect::<HashSet<_>>();
    let counts_by_allergen = counts_by_allergen(&food_and_allergens);
    let potential_allergens: HashSet<_> = counts_by_allergen.iter()
        .map(|(_allergen, counts)| {
            let max = counts.iter().map(|(_a,c)| c).max().unwrap();
            counts.iter()
                .filter(|(_ingredient, count)| *count == max)
                .map(|(ingredient, _count)| ingredient)
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();
    let allergene_free = all_ingredients.difference(&potential_allergens).collect::<HashSet<_>>();
    food_and_allergens.iter()
        .map(|(ingredients, _allergens)| ingredients)
        .flatten()
        .filter(|i| allergene_free.contains(i))
        .count() as u64
}

fn part_2(input: &str) -> String {
    let food_and_allergens = parse_input(input);
    let counts_by_allergen = counts_by_allergen(&food_and_allergens);
    let mut all_allergens = food_and_allergens.iter()
        .map(|(_food, allergens)| allergens)
        .flatten()
        .collect::<HashSet<_>>();
    let mut ingredients_list: BTreeMap<_, _> = BTreeMap::new();
    while !all_allergens.is_empty() {
        counts_by_allergen.iter().for_each(|(allergen, counts)| {
            let max = counts.iter().map(|(_a,c)| c).max().unwrap();
            let allergenic_ingredients: Vec<_> = counts.iter()
                .filter(|(_i, count)| *count == max)
                .filter(|(ingredient, _c)| ingredients_list.values().filter(|i| i == ingredient).count() == 0)
                .map(|(ingredient, _c)| ingredient.to_string())
                .collect();
            if allergenic_ingredients.len() == 1 {
                all_allergens.remove(allergen);
                ingredients_list.insert(allergen.to_string(), allergenic_ingredients.first().unwrap().to_string());
            }
        });
    }
    let ingredients_list = ingredients_list.values()
        .cloned()
        .collect::<Vec<_>>();
    ingredients_list.join(",")
}

fn parse_input(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input.lines()
        .map(|line| to_food_and_allergens(line))
        .collect()
}

fn counts_by_allergen(food_and_allergens: &Vec<(Vec<String>, Vec<String>)>) -> BTreeMap<String, HashMap<String, usize>> {
    food_and_allergens.iter()
        .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (ingredients, allergens)| {
            allergens.iter().for_each(|allergen| {
                acc.entry(allergen.clone()).or_insert(Vec::new()).extend(ingredients.iter().cloned())
            });
            acc
        })
        .iter()
        .fold(BTreeMap::<_, HashMap<_, _>>::new(), |mut acc, (allergen, ingredients)| {
            let counts = ingredients.iter()
                .fold(HashMap::<_, _>::new(), |mut counts, ingredient| {
                    *counts.entry(ingredient.clone()).or_default() += 1;
                    counts 
                });
            acc.insert(allergen.clone(), counts);
            acc
        })
}

fn to_food_and_allergens(line: &str) -> (Vec<String>, Vec<String>) {
    let parts = line.split(" (contains ").collect::<Vec<_>>();
    let ingredients = parts[0].split_whitespace()
        .map(|ingredient| ingredient.to_string())
        .collect::<Vec<_>>();
    let allergens = parts[1].split_whitespace()
        .map(|allergen| allergen.replace(")", "").replace(",", ""))
        .collect::<Vec<_>>();
    (ingredients, allergens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)";
        assert_eq!(part_1(input), 5);
    }

    #[test]
    fn part_2_test() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)";
        assert_eq!(part_2(input), "mxmxvkd,sqjhc,fvjkl".to_owned());
    }
}
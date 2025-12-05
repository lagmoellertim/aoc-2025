use crate::solver::day05::{parse_available_ingredients, parse_ingredient_ranges};
use crate::solver::registry::register_solver;

fn solve_day05_part01_puzzle(input: &str) -> String {
    let ingredient_ranges = parse_ingredient_ranges(input);
    let available_ingredients = parse_available_ingredients(input);

    let fresh_ingredients_count = available_ingredients
        .into_iter()
        .filter(|ingredient| {
            ingredient_ranges
                .iter()
                .any(|range| range.contains(ingredient))
        })
        .count();

    fresh_ingredients_count.to_string()
}

register_solver! {
    day: 5,
    part: 1,
    solver_fn: solve_day05_part01_puzzle
}

use crate::solver::day09::parse_red_tile_positions;
use crate::solver::registry::register_solver;
use itertools::Itertools;

fn solve_day09_part01_puzzle(input: &str) -> String {
    parse_red_tile_positions(input)
        .into_iter()
        .tuple_combinations()
        .map(|(corner_1, corner_2)| {
            ((corner_1.x - corner_2.x).abs() + 1) * ((corner_1.y - corner_2.y).abs() + 1)
        })
        .max()
        .unwrap()
        .to_string()
}

register_solver! {
    day: 9,
    part: 1,
    solver_fn: solve_day09_part01_puzzle
}

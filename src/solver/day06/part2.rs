use crate::solver::day06::{parse_problems, transpose, Operator};
use crate::solver::registry::register_solver;

fn solve_day06_part02_puzzle(input: &str) -> String {
    parse_problems(input)
        .into_iter()
        .map(|problem| {
            let char_grid = problem
                .padded_numbers
                .iter()
                .map(|padded_number| padded_number.chars().collect())
                .collect();

            let numbers = transpose(char_grid).into_iter().map(|chars| {
                chars
                    .into_iter()
                    .filter(|c| !c.is_whitespace())
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap()
            });

            match problem.operator {
                Operator::Add => numbers.sum::<i64>(),
                Operator::Multiply => numbers.product::<i64>(),
            }
        })
        .sum::<i64>()
        .to_string()
}

register_solver! {
    day: 6,
    part: 2,
    solver_fn: solve_day06_part02_puzzle
}

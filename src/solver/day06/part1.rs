use crate::solver::day06::{parse_problems, Operator};
use crate::solver::registry::register_solver;

fn solve_day06_part01_puzzle(input: &str) -> String {

    parse_problems(input)
        .into_iter()
        .map(|problem| {
            let numbers = problem
                .padded_numbers
                .into_iter()
                .map(|n| n.trim().parse::<i64>().unwrap());

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
    part: 1,
    solver_fn: solve_day06_part01_puzzle
}

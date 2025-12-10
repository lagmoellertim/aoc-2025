use crate::solver::day10::{parse_machines, Machine};
use crate::solver::registry::register_solver;
use itertools::Itertools;
use microlp::{ComparisonOp, OptimizationDirection, Problem};

fn get_minimal_button_presses_for_joltage(machine: &Machine) -> i64 {
    let mut problem = Problem::new(OptimizationDirection::Minimize);

    let max_button_presses = *machine.joltage_requirements.iter().max().unwrap();

    let button_vars = (0..machine.button_wiring_schematics.len())
        .map(|_| problem.add_integer_var(1.0, (0, max_button_presses as i32)))
        .collect::<Vec<_>>();

    let joltage_button_map = machine
        .button_wiring_schematics
        .iter()
        .enumerate()
        .flat_map(|(button_idx, wiring)| {
            wiring
                .iter()
                .map(move |joltage_idx| (joltage_idx, button_idx))
        })
        .into_group_map();

    for (joltage_idx, button_idxs) in joltage_button_map {
        problem.add_constraint(
            button_idxs
                .iter()
                .map(|button_idx| (button_vars[*button_idx], 1.0))
                .collect::<Vec<_>>(),
            ComparisonOp::Eq,
            machine.joltage_requirements[*joltage_idx as usize] as f64,
        );
    }

    problem
        .solve()
        .unwrap()
        .iter()
        .map(|(_, x)| x.round() as i64)
        .sum::<i64>()
}

fn solve_day10_part02_puzzle(input: &str) -> String {
    parse_machines(input)
        .into_iter()
        .map(|machine| get_minimal_button_presses_for_joltage(&machine))
        .sum::<i64>()
        .to_string()
}

register_solver! {
    day: 10,
    part: 2,
    solver_fn: solve_day10_part02_puzzle
}

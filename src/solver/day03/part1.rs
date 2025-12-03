use crate::solver::day03::parse_battery_banks;
use crate::solver::registry::register_solver;
use itertools::Itertools;

fn argmax<T: Ord>(slice: impl AsRef<[T]>) -> Option<usize> {
    slice
        .as_ref()
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|(_, value)| *value)
        .map(|(idx, _)| idx)
}

fn solve_day03_part01_puzzle(input: &str) -> String {
    parse_battery_banks(input)
        .iter()
        .map(|battery_bank| {
            let battery_idx_1 = argmax(battery_bank).unwrap();
            let battery_idx_2 = if battery_idx_1 == battery_bank.len() - 1 {
                argmax(&battery_bank[0..battery_idx_1]).unwrap()
            } else {
                argmax(&battery_bank[(battery_idx_1 + 1)..]).unwrap() + battery_idx_1 + 1
            };

            [battery_idx_1, battery_idx_2]
                .into_iter()
                .sorted()
                .map(|battery_idx| battery_bank[battery_idx])
                .reduce(|joltage_a, joltage_b| joltage_a * 10 + joltage_b)
                .unwrap()
        })
        .sum::<i64>()
        .to_string()
}

register_solver! {
    day: 3,
    part: 1,
    solver_fn: solve_day03_part01_puzzle
}

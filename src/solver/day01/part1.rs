use crate::solver::day01::parse_dial_movements;
use crate::solver::registry::register_solver;

fn solve_day01_part01_puzzle(input: &str) -> String {
    let mut position = 50;
    let mut count = 0;

    for movement in parse_dial_movements(input) {
        position = (position + movement).rem_euclid(100);

        if position == 0 {
            count += 1;
        }
    }

    count.to_string()
}

register_solver! {
    day: 1,
    part: 1,
    solver_fn: solve_day01_part01_puzzle
}

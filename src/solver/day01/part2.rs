use crate::solver::day01::parse_dial_movements;
use crate::solver::registry::register_solver;

pub fn solve_day01_part02_puzzle(input: &str) -> String {
    let mut current_position: i32 = 50;
    let mut count = 0;

    for movement in parse_dial_movements(input) {
        count += (current_position + movement).div_euclid(100).abs();
        let next_position = (current_position + movement).rem_euclid(100);

        if movement < 0 {
            if current_position == 0 {
                count -= 1;
            }

            if next_position == 0 {
                count += 1;
            }
        }

        current_position = next_position;
    }

    count.to_string()
}

register_solver! {
    day: 1,
    part: 2,
    solver_fn: solve_day01_part02_puzzle
}

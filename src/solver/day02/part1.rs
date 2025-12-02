use crate::solver::day02::parse_id_ranges;
use crate::solver::registry::register_solver;

fn solve_day02_part01_puzzle(input: &str) -> String {
    let mut sum = 0;

    for id_range in parse_id_ranges(input) {
        for id in id_range.lower_id..=id_range.upper_id {
            let chars = id.to_string().chars().collect::<Vec<_>>();

            if chars.len() % 2 == 1 {
                continue;
            }

            let is_invalid = (chars[0..chars.len() / 2] == chars[chars.len() / 2..chars.len()]);

            if is_invalid {
                sum += id;
            }
        }
    }

    sum.to_string()
}

register_solver! {
    day: 2,
    part: 1,
    solver_fn: solve_day02_part01_puzzle
}

use crate::solver::day02::parse_id_ranges;
use crate::solver::registry::register_solver;

fn solve_day02_part02_puzzle(input: &str) -> String {
    let mut sum = 0;

    for id_range in parse_id_ranges(input) {
        for id in id_range.lower_id..=id_range.upper_id {
            let chars = id.to_string().chars().collect::<Vec<_>>();

            let mut possible_chunk_sizes =
                (1..=chars.len() / 2).filter(|chunk_size| chars.len() % chunk_size == 0);

            let is_invalid = possible_chunk_sizes.any(|chunk_size| {
                chars
                    .chunks(chunk_size)
                    .collect::<Vec<_>>()
                    .windows(2)
                    .all(|window| window[0] == window[1])
            }); 

            if is_invalid {
                sum += id;
            }
        }
    }

    sum.to_string()
}

register_solver! {
    day: 2,
    part: 2,
    solver_fn: solve_day02_part02_puzzle
}

use crate::solver::day04::{parse_paper_roll_map, Cell, ADJACENT_POSITION_OFFSETS};
use crate::solver::registry::register_solver;

fn solve_day04_part01_puzzle(input: &str) -> String {
    let paper_roll_map = parse_paper_roll_map(input);

    let mut reachable_paper_rolls_count: i64 = 0;

    for ((y, x), cell) in paper_roll_map.indexed_iter() {
        let Cell::Paper = cell else {
            continue;
        };

        let adjacent_paper_rolls_count = ADJACENT_POSITION_OFFSETS
            .iter()
            .filter_map(|&(offset_y, offset_x)| {
                let adjacent_x = usize::try_from(x as i64 + offset_x).ok()?;
                let adjacent_y = usize::try_from(y as i64 + offset_y).ok()?;
                paper_roll_map.get((adjacent_y, adjacent_x))
            })
            .filter(|cell| matches!(cell, Cell::Paper))
            .count();

        if adjacent_paper_rolls_count < 4 {
            reachable_paper_rolls_count += 1;
        }
    }

    reachable_paper_rolls_count.to_string()
}

register_solver! {
    day: 4,
    part: 1,
    solver_fn: solve_day04_part01_puzzle
}

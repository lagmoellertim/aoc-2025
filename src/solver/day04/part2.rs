use crate::solver::day04::{parse_paper_roll_map, Cell, ADJACENT_POSITION_OFFSETS};
use crate::solver::registry::register_solver;
use std::collections::HashSet;

fn solve_day04_part02_puzzle(input: &str) -> String {
    let paper_roll_map = parse_paper_roll_map(input);

    let mut paper_roll_positions = paper_roll_map
        .indexed_iter()
        .filter(|(_, cell)| matches!(cell, Cell::Paper))
        .map(|((x, y), _)| (y as i64, x as i64))
        .collect::<HashSet<_>>();

    let mut reachable_paper_rolls_count_changed = true;
    let mut reachable_paper_rolls_count = 0;

    while reachable_paper_rolls_count_changed {
        reachable_paper_rolls_count_changed = false;
        for (y, x) in paper_roll_positions.clone() {
            let adjacent_paper_roll_count = ADJACENT_POSITION_OFFSETS
                .iter()
                .filter(|(offset_x, offset_y)| {
                    paper_roll_positions.contains(&(*offset_y + y, *offset_x + x))
                })
                .count();

            if adjacent_paper_roll_count < 4 {
                paper_roll_positions.remove(&(y, x));
                reachable_paper_rolls_count_changed = true;
                reachable_paper_rolls_count += 1;
            }
        }
    }

    reachable_paper_rolls_count.to_string()
}

register_solver! {
    day: 4,
    part: 2,
    solver_fn: solve_day04_part02_puzzle
}

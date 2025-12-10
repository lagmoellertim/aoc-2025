use crate::solver::day07::{find_start_beam_and_remaining_rows, parse_tachyon_manifold_diagram, Cell};
use crate::solver::registry::register_solver;
use std::collections::HashSet;

fn solve_day07_part01_puzzle(input: &str) -> String {
    let diagram = parse_tachyon_manifold_diagram(input);
    let (start_beam, remaining_rows) = find_start_beam_and_remaining_rows(&diagram);

    let mut beams = HashSet::from([start_beam]);

    let mut beam_splits = 0;

    for row in remaining_rows {
        for beam in beams.clone() {
            if let Cell::Splitter = row[beam] {
                beams.remove(&beam);
                beam_splits += 1;

                beams.insert(beam - 1);
                beams.insert(beam + 1);
            }
        }
    }

    beam_splits.to_string()
}

register_solver! {
    day: 7,
    part: 1,
    solver_fn: solve_day07_part01_puzzle
}

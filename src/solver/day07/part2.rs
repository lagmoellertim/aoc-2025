use crate::solver::day07::{find_start_beam_and_remaining_rows, parse_tachyon_manifold_diagram, Cell};
use crate::solver::registry::register_solver;
use std::collections::HashMap;

fn solve_day07_part02_puzzle(input: &str) -> String {
    let diagram = parse_tachyon_manifold_diagram(input);
    let (start_beam, remaining_rows) = find_start_beam_and_remaining_rows(&diagram);

    let mut beams = HashMap::from([(start_beam, 1)]);

    for row in remaining_rows {
        for (beam, quantum_superposition_count) in beams.clone() {
            match row[beam] {
                Cell::Splitter => {
                    beams.remove(&beam);
                    *beams.entry(beam - 1).or_insert(0) += quantum_superposition_count;
                    *beams.entry(beam + 1).or_insert(0) += quantum_superposition_count;
                }
                _ => {}
            }
        }
    }

    beams.into_iter().map(|(_, quantum_superposition_count)| quantum_superposition_count).sum::<i64>().to_string()
}

register_solver! {
    day: 7,
    part: 2,
    solver_fn: solve_day07_part02_puzzle
}

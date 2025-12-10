use crate::solver::day10::{parse_machines, Machine};
use crate::solver::registry::register_solver;
use itertools::Itertools;

fn get_minimal_button_presses_for_indicator_lights(machine: &Machine) -> i64 {
    (1..machine.button_wiring_schematics.len())
        .into_iter()
        .find_map(|k| {
            machine
                .button_wiring_schematics
                .iter()
                .combinations(k)
                .find(|button_wiring_schematics| {
                    let mut current_indicator_lights =
                        vec![false; machine.indicator_light_diagram.len()];

                    for &button_wiring_schematic in button_wiring_schematics {
                        for &indicator_light_idx in button_wiring_schematic {
                            current_indicator_lights[indicator_light_idx as usize] ^= true;
                        }
                    }

                    current_indicator_lights == machine.indicator_light_diagram
                })
                .map(|_| k as i64)
        })
        .unwrap()
}

fn solve_day10_part01_puzzle(input: &str) -> String {
    parse_machines(input)
        .into_iter()
        .map(|machine| get_minimal_button_presses_for_indicator_lights(&machine))
        .sum::<i64>()
        .to_string()
}

register_solver! {
    day: 10,
    part: 1,
    solver_fn: solve_day10_part01_puzzle
}

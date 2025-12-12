use crate::solver::day12::{parse_packaging_manifest, GridCell};
use crate::solver::registry::register_solver;

fn solve_day12_part01_puzzle(input: &str) -> String {
    let packaging_manifest = parse_packaging_manifest(input);

    let shape_areas = packaging_manifest
        .shapes
        .iter()
        .map(|shape| {
            shape
                .iter()
                .flatten()
                .filter(|f| matches!(f, GridCell::Present))
                .count() as i64
        })
        .collect::<Vec<_>>();

    packaging_manifest
        .regions
        .iter()
        .filter(|region| {
            let region_area = region.width * region.height;

            let min_presents_area = region
                .required_quantities
                .iter()
                .map(|(shape_idx, quantity)| (shape_areas[*shape_idx]) * quantity)
                .sum::<i64>();

            let max_presents_area = region
                .required_quantities
                .iter()
                .map(|(_, quantity)| 9 * quantity)
                .sum::<i64>();

            if min_presents_area >= region_area {
                return false;
            }

            if max_presents_area <= region_area {
                return true;
            };

            unimplemented!("Now we would need a real packing solver here ... 💀")
        })
        .count()
        .to_string()
}

register_solver! {
    day: 12,
    part: 1,
    solver_fn: solve_day12_part01_puzzle
}

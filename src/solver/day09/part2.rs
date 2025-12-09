use crate::solver::day09::{parse_red_tile_positions, Point};
use crate::solver::registry::register_solver;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::ops::Range;

fn do_edges_intersect(
    edges_map: &BTreeMap<i64, Vec<(i64, i64)>>,
    parallel_bounds: Range<i64>,
    perpendicular_bounds: Range<i64>,
) -> bool {
    edges_map.range(parallel_bounds).any(|(_, edges)| {
        edges.iter().any(|&(start, end)| {
            let (edge_min, edge_max) = (start.min(end), start.max(end));
            edge_max > perpendicular_bounds.start && edge_min < perpendicular_bounds.end
        })
    })
}

fn is_valid_rectangle(
    corner_1: &Point,
    corner_2: &Point,
    vertical_edges_map: &BTreeMap<i64, Vec<(i64, i64)>>,
    horizontal_edges_map: &BTreeMap<i64, Vec<(i64, i64)>>,
) -> bool {
    let (min_x, max_x) = (corner_1.x.min(corner_2.x), corner_1.x.max(corner_2.x));
    let (min_y, max_y) = (corner_1.y.min(corner_2.y), corner_1.y.max(corner_2.y));

    let (center_x, center_y) = ((min_x + max_x) / 2, (min_y + max_y) / 2);

    let is_center_inside_polygon = vertical_edges_map
        .range((center_x + 1)..)
        .flat_map(|(_, edges)| edges)
        .filter(|&&(edge_y_1, edge_y_2)| {
            let (edge_min_y, edge_max_y) = (edge_y_1.min(edge_y_2), edge_y_1.max(edge_y_2));
            center_y > edge_min_y && center_y < edge_max_y
        })
        .count()
        % 2
        == 1;

    if !is_center_inside_polygon {
        return false;
    }

    if min_x + 1 < max_x && do_edges_intersect(vertical_edges_map, (min_x + 1)..max_x, min_y..max_y) {
        return false;
    }

    if min_y + 1 < max_y && do_edges_intersect(horizontal_edges_map, (min_y + 1)..max_y, min_x..max_x)
    {
        return false;
    }

    true
}

fn solve_day09_part02_puzzle(input: &str) -> String {
    let red_tile_positions = parse_red_tile_positions(input);

    let mut vertical_edges_map: BTreeMap<i64, Vec<(i64, i64)>> = BTreeMap::new();
    let mut horizontal_edges_map: BTreeMap<i64, Vec<(i64, i64)>> = BTreeMap::new();

    for (point_1, point_2) in red_tile_positions.iter().circular_tuple_windows() {
        if point_1.x == point_2.x {
            vertical_edges_map
                .entry(point_1.x)
                .or_default()
                .push((point_1.y, point_2.y));
        } else if point_1.y == point_2.y {
            horizontal_edges_map
                .entry(point_1.y)
                .or_default()
                .push((point_1.x, point_2.x));
        }
    }

    red_tile_positions
        .iter()
        .tuple_combinations()
        .filter(|(corner_1, corner_2)| {
            is_valid_rectangle(
                corner_1,
                corner_2,
                &vertical_edges_map,
                &horizontal_edges_map,
            )
        })
        .map(|(corner_1, corner_2)| {
            ((corner_1.x - corner_2.x).abs() + 1) * ((corner_1.y - corner_2.y).abs() + 1)
        })
        .max()
        .unwrap_or(0)
        .to_string()
}

register_solver! {
    day: 9,
    part: 2,
    solver_fn: solve_day09_part02_puzzle
}

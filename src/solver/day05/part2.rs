use crate::solver::day05::parse_ingredient_ranges;
use crate::solver::registry::register_solver;
use itertools::Itertools;
use std::ops::RangeInclusive;

#[derive(Debug)]
enum Event {
    Enter,
    Leave,
}

fn merge_ranges(overlapping_ranges: impl AsRef<[RangeInclusive<i64>]>) -> Vec<RangeInclusive<i64>> {
    let positions_with_events = overlapping_ranges
        .as_ref()
        .iter()
        .flat_map(|range| [(*range.start(), Event::Enter), (*range.end(), Event::Leave)])
        .sorted_by_key(|(position, _)| *position)
        .chunk_by(|(position, _)| *position)
        .into_iter()
        .map(|(position, group)| (position, group.map(|(_, event)| event).collect::<Vec<_>>()))
        .collect::<Vec<_>>();

    let mut merged_ranges = Vec::new();
    let mut current_start = None;
    let mut overlap_count = 0;

    for (position, events) in positions_with_events {
        let enter_events = events.iter().filter(|e| matches!(e, Event::Enter)).count();
        let leave_events = events.iter().filter(|e| matches!(e, Event::Leave)).count();

        if overlap_count == 0 && enter_events > 0 {
            current_start = Some(position);
        }

        overlap_count += enter_events - leave_events;

        if let Some(start) = current_start {
            if overlap_count == 0 {
                merged_ranges.push(start..=position);
                current_start = None;
            }
        }
    }

    merged_ranges
}

fn solve_day05_part02_puzzle(input: &str) -> String {
    let ingredient_ranges = parse_ingredient_ranges(input);

    let fresh_ingredients_count = merge_ranges(&ingredient_ranges)
        .iter()
        .map(|range| range.try_len().unwrap())
        .sum::<usize>();

    fresh_ingredients_count.to_string()
}

register_solver! {
    day: 5,
    part: 2,
    solver_fn: solve_day05_part02_puzzle
}

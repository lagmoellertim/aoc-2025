use std::ops::RangeInclusive;

mod part1;
mod part2;

fn parse_ingredient_ranges(input: &str) -> Vec<RangeInclusive<i64>> {
    input
        .lines()
        .filter(|line| line.contains("-"))
        .map(|line| {
            let items = line
                .split("-")
                .map(|part| part.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            items[0]..=items[1]
        })
        .collect::<Vec<_>>()
}

fn parse_available_ingredients(input: &str) -> Vec<i64> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.contains("-"))
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

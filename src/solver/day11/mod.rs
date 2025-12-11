use std::collections::{HashMap, HashSet};

mod part1;
mod part2;

type Connections = HashMap<String, HashSet<String>>;

fn parse_connections(input: &str) -> Connections {
    input
        .lines()
        .map(|line| {
            let parts = line.split(":").collect::<Vec<_>>();

            (
                parts[0].trim().to_string(),
                parts[1]
                    .split_whitespace()
                    .filter(|s| !s.is_empty())
                    .map(|s| s.trim().to_string())
                    .collect(),
            )
        })
        .collect()
}

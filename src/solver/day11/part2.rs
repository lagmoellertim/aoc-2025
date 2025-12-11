use crate::solver::day11::{parse_connections, Connections};
use crate::solver::registry::register_solver;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

fn count_paths(start: &str, end: &str, connections: &Connections) -> i64 {
    let mut depths: HashMap<&str, usize> = HashMap::from([(start, 0)]);
    let mut queue = VecDeque::from([start]);
    let mut in_queue = HashSet::from([start]);

    while let Some(current_node) = queue.pop_front() {
        in_queue.remove(current_node);

        if let Some(next_nodes) = connections.get(current_node) {
            let current_depth = depths[current_node];

            for next_node in next_nodes {
                let next_node = next_node.as_str();

                depths
                    .entry(next_node)
                    .and_modify(|next_depth| *next_depth = (*next_depth).max(current_depth + 1))
                    .or_insert(current_depth + 1);

                if in_queue.insert(next_node) {
                    queue.push_back(next_node);
                }
            }
        }
    }

    let mut node_path_counts: HashMap<&str, i64> = HashMap::from([(start, 1)]);

    let sorted_nodes = depths
        .into_iter()
        .sorted_by_key(|(_, depth)| *depth)
        .map(|(node, _)| node);

    for current_node in sorted_nodes {
        if let Some(next_nodes) = connections.get(current_node) {
            let current_node_path_count = *node_path_counts.get(current_node).unwrap();

            for next_node in next_nodes {
                let next_node = next_node.as_str();
                let next_node_path_count = *node_path_counts.get(&next_node).unwrap_or(&0);

                node_path_counts.insert(next_node, current_node_path_count + next_node_path_count);
            }
        }
    }

    *node_path_counts.get(end).unwrap_or(&0)
}

fn solve_day11_part02_puzzle(input: &str) -> String {
    let connections = parse_connections(input);

    let path_1_count = count_paths("svr", "dac", &connections)
        * count_paths("dac", "fft", &connections)
        * count_paths("fft", "out", &connections);

    let path_2_count = count_paths("svr", "fft", &connections)
        * count_paths("fft", "dac", &connections)
        * count_paths("dac", "out", &connections);

    (path_1_count + path_2_count).to_string()
}

register_solver! {
    day: 11,
    part: 2,
    solver_fn: solve_day11_part02_puzzle
}

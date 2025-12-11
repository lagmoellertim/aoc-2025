use crate::solver::day11::{parse_connections, Connections};
use crate::solver::registry::register_solver;

fn count_paths_recursive(current: &str, end: &str, connections: &Connections) -> i64 {
    if current == end {
        return 1;
    }

    connections[current]
        .iter()
        .map(|next| count_paths_recursive(next, end, connections))
        .sum()
}

fn solve_day11_part01_puzzle(input: &str) -> String {
    count_paths_recursive("you", "out", &parse_connections(input)).to_string()
}

register_solver! {
    day: 11,
    part: 1,
    solver_fn: solve_day11_part01_puzzle
}

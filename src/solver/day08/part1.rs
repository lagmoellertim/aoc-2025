use crate::solver::day08::parse_coordinates;
use crate::solver::registry::register_solver;
use itertools::Itertools;
use std::collections::HashSet;

fn solve_day08_part01_puzzle(input: &str) -> String {
    const CONNECTIONS: usize = 1000;

    let connections = parse_coordinates(input)
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((i, coordinate_1), (j, coordinate_2))| (coordinate_1.distance(coordinate_2), i, j))
        .sorted_by(|(d1, _, _), (d2, _, _)| d1.total_cmp(d2));

    let mut clusters: Vec<HashSet<usize>> = vec![];

    for (_, i, j) in connections.take(CONNECTIONS) {
        let cluster_i = clusters.iter().position(|cluster| cluster.contains(&i));
        let cluster_j = clusters.iter().position(|cluster| cluster.contains(&j));

        match (cluster_i, cluster_j) {
            (Some(cluster_i), Some(cluster_j)) => {
                if cluster_i != cluster_j {
                    let item = clusters.remove(cluster_i.max(cluster_j));
                    clusters[cluster_i.min(cluster_j)].extend(item);
                }
            }
            (Some(cluster_i), _) => {
                clusters[cluster_i].insert(j);
            }
            (_, Some(cluster_j)) => {
                clusters[cluster_j].insert(i);
            }
            (_, _) => clusters.push(HashSet::from([i, j])),
        }
    }

    clusters
        .iter()
        .map(|cluster| cluster.len())
        .sorted()
        .rev()
        .take(3)
        .product::<usize>()
        .to_string()
}

register_solver! {
    day: 8,
    part: 1,
    solver_fn: solve_day08_part01_puzzle
}

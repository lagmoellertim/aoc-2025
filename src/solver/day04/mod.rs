use ndarray::Array2;

mod part1;
mod part2;

const ADJACENT_POSITION_OFFSETS: &[(i64, i64)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug)]
enum Cell {
    Paper,
    None,
}

fn parse_paper_roll_map(input: &str) -> Array2<Cell> {
    let items: Vec<Vec<Cell>> = input
        .split("\n")
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '@' => Cell::Paper,
                    '.' => Cell::None,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let rows = items.len();
    let cols = items.first().map_or(0, |row| row.len());

    let flat_data = items.into_iter().flatten().collect();
    Array2::from_shape_vec((rows, cols), flat_data).unwrap()
}

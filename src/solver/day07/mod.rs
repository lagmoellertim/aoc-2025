mod part1;
mod part2;

#[derive(Debug)]
enum Cell {
    Start,
    Splitter,
    None,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            'S' => Cell::Start,
            '^' => Cell::Splitter,
            '.' => Cell::None,
            _ => unreachable!()
        }
    }
}

fn find_start_beam_and_remaining_rows(diagram: &Vec<Vec<Cell>>) -> (usize, &[Vec<Cell>]) {
    let mut start_position = None;
    for (row_index, row) in diagram.iter().enumerate() {
        for (column_index, cell) in row.iter().enumerate() {
            if matches!(cell, Cell::Start) {
                start_position = Some((row_index, column_index));
            }
        }
    };

    let (start_row, start_column) = start_position.unwrap();

    (start_column, &diagram[start_row + 1..])
}

fn parse_tachyon_manifold_diagram(input: &str) -> Vec<Vec<Cell>> {
    input.lines().into_iter().map(|line| line.chars().map(|c| c.into()).collect()).collect()
}

use std::iter;

mod part1;
mod part2;

#[derive(Debug)]
enum Operator {
    Multiply,
    Add,
}

#[derive(Debug)]
struct Problem {
    padded_numbers: Vec<String>,
    operator: Operator,
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if v.is_empty() {
        return vec![];
    }
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect())
        .collect()
}

fn parse_problems(input: &str) -> Vec<Problem> {
    let lines = input.lines().collect::<Vec<_>>();

    let columns = lines
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_, c)| !c.is_whitespace())
        .map(|(i, _)| i)
        .chain(iter::once(lines[0].len() + 1))
        .collect::<Vec<_>>()
        .windows(2)
        .map(|w| w[0]..w[1] - 1)
        .collect::<Vec<_>>();

    let columns = transpose(
        lines
            .into_iter()
            .map(|line| columns.iter().map(|column| &line[column.clone()]).collect())
            .collect(),
    );

    columns
        .into_iter()
        .map(|column| {
            let (last, other) = column.split_last().unwrap();

            let operator = match last.trim() {
                "*" => Operator::Multiply,
                "+" => Operator::Add,
                _ => unreachable!(),
            };

            let padded_numbers = other
                .into_iter()
                .map(|padded_number| padded_number.to_string())
                .collect::<Vec<_>>();

            Problem {
                padded_numbers,
                operator,
            }
        })
        .collect::<Vec<_>>()
}

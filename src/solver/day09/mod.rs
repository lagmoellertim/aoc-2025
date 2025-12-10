use std::ops::Add;

mod part1;
mod part2;

#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn parse_red_tile_positions(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let numbers = line
                .split(",")
                .map(|part| part.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            Point {
                x: numbers[0],
                y: numbers[1],
            }
        })
        .collect()
}

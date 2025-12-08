mod part1;
mod part2;

#[derive(Debug, PartialEq, Clone)]
struct Coordinate {
    x: f64,
    y: f64,
    z: f64,
}

impl Coordinate {
    fn distance(&self, other: &Coordinate) -> f64 {
        ((self.x - other.x).powi(2)
            + (self.y - other.y).powi(2)
            + (self.z - other.z).powi(2))
        .sqrt()
    }
}

fn parse_coordinates(input: &str) -> Vec<Coordinate> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let numbers = line
                .split(",")
                .map(|part| part.parse::<f64>().unwrap())
                .collect::<Vec<_>>();
            Coordinate {
                x: numbers[0],
                y: numbers[1],
                z: numbers[2],
            }
        })
        .collect()
}

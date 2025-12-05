mod part1;
mod part2;

fn parse_dial_movements(input: &str) -> impl Iterator<Item=i32> {
    input.lines().map(|line| {
        let number = line[1..].parse::<i32>().unwrap();

        match line.chars().next().unwrap() {
            'L' => -number,
            'R' => number,
            _ => unreachable!()
        }
    })
}

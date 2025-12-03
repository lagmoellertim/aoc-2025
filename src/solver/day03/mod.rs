mod part1;
mod part2;

fn parse_battery_banks(input: &str) -> Vec<Vec<i64>> {
    input
        .split("\n")
        .map(|line| {
            line.trim().chars()
                .map(|c| c.to_digit(10).unwrap().into())
                .collect()
        })
        .collect()
}

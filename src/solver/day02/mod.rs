mod part1;
mod part2;

#[derive(Debug)]
struct IdRange {
    lower_id: i64,
    upper_id: i64,
}

fn parse_id_ranges(input: &str) -> Vec<IdRange> {
    input
        .replace("\n", "")
        .split(",")
        .map(|part| {
            let parts = part.trim().split("-").collect::<Vec<_>>();

            IdRange {
                lower_id: parts[0].parse().unwrap(),
                upper_id: parts[1].parse().unwrap(),
            }
        })
        .collect()
}
 
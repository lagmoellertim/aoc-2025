mod part1;
mod part2;

#[derive(Debug)]
struct Machine {
    indicator_light_diagram: Vec<bool>,
    button_wiring_schematics: Vec<Vec<i64>>,
    joltage_requirements: Vec<i64>,
}

fn parse_machines(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let indicator_light_diagram = line
                .trim_matches(|c| c != '[' && c != ']')
                .trim_matches(['[', ']'])
                .chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => unreachable!(),
                })
                .collect();

            let button_wiring_schematics = line
                .trim_matches(|c| c != '(' && c != ')')
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|part| {
                    part.trim_matches(['(', ')'])
                        .split(",")
                        .map(|n| n.parse::<i64>().unwrap())
                        .collect()
                })
                .collect();

            let joltage_requirements = line
                .trim_matches(|c| c != '{' && c != '}')
                .trim_matches(['{', '}'])
                .split(",")
                .map(|n| n.parse::<i64>().unwrap())
                .collect();

            Machine {
                indicator_light_diagram,
                button_wiring_schematics,
                joltage_requirements,
            }
        })
        .collect()
}

use crate::solver::day03::parse_battery_banks;
use crate::solver::registry::register_solver;

fn solve_day03_part02_puzzle(input: &str) -> String {
    let mut total_joltage = 0;

    for battery_bank in parse_battery_banks(input) {
        const REQUIRED_BATTERY_COUNT: i64 = 12;

        let mut battery_buffer = vec![];

        for (battery_idx, battery_joltage) in battery_bank.iter().enumerate() {
            loop {
                let Some(last_battery_in_buffer_joltage) = battery_buffer.last() else {
                    break;
                };

                let has_larger_joltage_than_last_battery_in_buffer = battery_joltage > last_battery_in_buffer_joltage;
                let can_still_fill_up_battery_buffer = (battery_bank.len() - battery_idx) as i64
                    >= REQUIRED_BATTERY_COUNT - (battery_buffer.len() as i64 - 1);

                if has_larger_joltage_than_last_battery_in_buffer && can_still_fill_up_battery_buffer {
                    battery_buffer.pop();
                } else {
                    break;
                }
            }

            battery_buffer.push(*battery_joltage);
        }

        let num = battery_buffer
            .into_iter()
            .take(12)
            .reduce(|x, y| x * 10 + y)
            .unwrap();

        total_joltage += num;
    }

    total_joltage.to_string()
}

register_solver! {
    day: 3,
    part: 2,
    solver_fn: solve_day03_part02_puzzle
}

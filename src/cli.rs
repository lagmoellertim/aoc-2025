use crate::solver::registry;
use crate::solver::registry::get_solver_fn;
use anyhow::anyhow;
use clap::Parser;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: Option<usize>,

    #[arg(short, long)]
    part: Option<usize>,

    #[arg(short, long)]
    input: Option<PathBuf>,

    #[arg(short, long, default_value = "1")]
    runs: usize,
}

fn get_valid_puzzle_day(args: &Args) -> anyhow::Result<usize> {
    let mut available_days = registry::get_available_days();
    available_days.sort();

    let day = args.day.unwrap_or_else(|| {
        let selected_idx = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Day")
            .items(&available_days)
            .default(0)
            .interact()
            .unwrap();

        available_days[selected_idx]
    });

    if !available_days.contains(&day) {
        return Err(anyhow!("No solver for puzzle of day {} found", day));
    }

    Ok(day)
}

fn get_valid_puzzle_part(args: &Args, day: usize) -> anyhow::Result<usize> {
    let mut available_parts = registry::get_available_parts_for_day(day);
    available_parts.sort();


    let part = args.part.unwrap_or_else(|| {
        let selected_idx = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Part")
            .items(&available_parts)
            .default(0)
            .interact()
            .unwrap();

        available_parts[selected_idx]
    });

    if !available_parts.contains(&part) {
        return Err(anyhow!(
            "No solver for puzzle of day {} - part {} found",
            day,
            part
        ));
    }

    Ok(part)
}

fn get_puzzle_inputs(day: usize) -> anyhow::Result<Vec<PathBuf>> {
    let path = PathBuf::from(format!("puzzle_inputs/day{:02}", day));

    let files = fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();

    Ok(files)
}

fn get_valid_puzzle_input_file(args: &Args, day: usize) -> anyhow::Result<PathBuf> {
    let input = args.input.clone().unwrap_or_else(|| {
        let inputs = get_puzzle_inputs(day).unwrap();

        let selected_idx = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Puzzle Input")
            .items(
                inputs
                    .iter()
                    .map(|path| path.file_name().unwrap().to_string_lossy().to_string())
                    .collect::<Vec<String>>(),
            )
            .default(0)
            .interact()
            .unwrap();

        inputs[selected_idx].clone()
    });

    Ok(input)
}

pub fn run_cli() -> anyhow::Result<()> {
    let args = Args::parse();

    let day = get_valid_puzzle_day(&args)?;
    let part = get_valid_puzzle_part(&args, day)?;

    let solver_fn = get_solver_fn(day, part).ok_or(anyhow!(
        "No solver for puzzle of day {} - part {} found",
        day,
        part
    ))?;

    let input_file = get_valid_puzzle_input_file(&args, day)?;

    let input = fs::read_to_string(&input_file)?;

    let mut benchmarked_times = Vec::new();

    let mut solution = String::new();

    for _ in 0..args.runs {
        let start = Instant::now();
        solution = solver_fn(&input);
        let elapsed = start.elapsed();

        benchmarked_times.push(elapsed);
    }

    println!(
        "🎁 Solution: {}",
        solution
    );

    println!("{}", "━".repeat(40));

    benchmarked_times.sort();
    let fastest_time = benchmarked_times[0];
    println!("🚀 Performance: {:?} (Fastest of {} runs)", fastest_time, args.runs);

    #[cfg(debug_assertions)]
    let build_type = "Debug Build";

    #[cfg(not(debug_assertions))]
    let build_type = "Release Build";

    println!("⚙️ Build Type: {}", build_type);

    Ok(())
}

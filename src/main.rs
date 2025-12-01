use crate::cli::run_cli;

mod cli;
mod solver;

fn main() -> anyhow::Result<()> {
    run_cli()
}

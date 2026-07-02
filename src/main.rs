mod cli;
mod export;
mod models;
mod normalize;
mod pipeline;
mod sources;
mod validate;

use clap::Parser;
use cli::{Cli, Command, Country};
use pipeline::normalize_indonesia;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Normalize { country } => match country {
            Country::Indonesia => normalize_indonesia(),
        },
    }
}

mod cache;
mod cli;
mod export;
mod fetch;
mod models;
mod normalize;
mod pipeline;
mod sources;
mod validate;

use clap::Parser;
use cli::{Cli, Command, Country};
use pipeline::{
    fetch_indonesia_sources, normalize_indonesia, parse_bps_indonesia_sources,
    print_indonesia_sources,
};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Normalize { country } => match country {
            Country::Indonesia => normalize_indonesia(),
        },
        Command::Sources { country } => {
            match country {
                Country::Indonesia => print_indonesia_sources(),
            }

            Ok(())
        }
        Command::Fetch { country, level } => match country {
            Country::Indonesia => fetch_indonesia_sources(level),
        },
        Command::ParseBps { country } => match country {
            Country::Indonesia => parse_bps_indonesia_sources(),
        },
    }
}

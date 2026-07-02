mod cli;
mod export;
mod models;
mod normalize;
mod sources;
mod validate;

use clap::Parser;
use cli::{Cli, Command, Country};
use export::{export_regions_to_csv, export_regions_to_json};
use normalize::normalize_indonesia_data;
use sources::indonesia::load_local_data;
use validate::validate_regions;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Normalize { country } => match country {
            Country::Indonesia => normalize_indonesia(),
        },
    }
}

fn normalize_indonesia() -> anyhow::Result<()> {
    let data = load_local_data()?;

    let regions = normalize_indonesia_data(data);

    validate_regions(&regions)
        .map_err(|errors| anyhow::anyhow!("validation failed:\n{}", errors.join("\n")))?;

    export_regions_to_json(&regions, "output/indonesia/regions.json")?;
    export_regions_to_csv(&regions, "output/indonesia/regions.csv")?;

    println!("Exported {} regions", regions.len());
    println!("JSON: output/indonesia/regions.json");
    println!("CSV: output/indonesia/regions.csv");

    Ok(())
}

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "region_squirrel")]
#[command(about = "Normalize administrative region data")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Normalize {
        #[arg(value_enum)]
        country: Country,
    },
    Sources {
        #[arg(value_enum)]
        country: Country,
    },
    Fetch {
        #[arg(value_enum)]
        country: Country,

        #[arg(value_enum, default_value_t = FetchLevel::All)]
        level: FetchLevel,
    },
    ParseBps {
        #[arg(value_enum)]
        country: Country,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Country {
    Indonesia,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum FetchLevel {
    Provinces,
    Regencies,
    Districts,
    All,
}

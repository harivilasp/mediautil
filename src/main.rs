use anyhow::Result;
use clap::Parser;
use mediautil::{cli::Cli, commands};

fn main() -> Result<()> {
    let cli = Cli::parse();
    commands::run(cli.command)
}

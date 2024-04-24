#![warn(clippy::pedantic)]

use clap::Parser;

mod cli;
use cli::Cmd;

fn main() -> anyhow::Result<()> {
    let args = cli::CliArgs::parse();

    args.run()
}

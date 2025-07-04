mod cli;
mod impls;
mod error;
mod utils;

use crate::cli::FunCli;
use clap::Parser;

fn main() {
    let cli = FunCli::parse();
    let commands = cli.command;
    commands.run();
}




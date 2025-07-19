// src/main.rs
/*
 * Main executable for CollaborationsoftwareCore
 */

use clap::Parser;
use collaborationsoftwarecore::{Result, run};

#[derive(Parser)]
#[command(version, about = CollaborationsoftwareCore - A Rust implementation)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}

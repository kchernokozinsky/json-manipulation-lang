use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct JmlCli {
    #[command(subcommand)]
    pub command: JmlCommand,
}

#[derive(Subcommand)]
pub enum JmlCommand {
    Run {
        #[arg(short, long)]
        file: PathBuf,

        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

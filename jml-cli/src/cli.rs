use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "JML Command-Line Tool.",
    before_help = "                    
    __ _____ __    
 __|  |     |  | 
|  |  | | | |  |__ 
|_____|_|_|_|_____|",
    after_help = "For more information, visit https://github.com/kchernokozinsky/json-manipulation-lang"
)]
/// JML Command-Line Tool
pub struct JmlCli {
    #[command(subcommand)]
    pub command: JmlCommand,
}

#[derive(Subcommand)]

pub enum JmlCommand {
    /// Run the JML parser and evaluator on a given file.
    Run {
        /// Path to the JML source file to be parsed and evaluated.
        #[arg(short, long, help = "Input JML file to process.")]
        file: PathBuf,

        /// Optional path to write the output as JSON.
        ///
        /// If provided, the result of evaluating the JML file will be written
        /// to this output file in JSON format.
        #[arg(short, long, help = "Output file for JSON result.")]
        output: Option<PathBuf>,
    },
}

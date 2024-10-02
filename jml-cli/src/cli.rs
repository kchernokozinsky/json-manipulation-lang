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

    /// Enable or disable logging
    #[arg(short, long, help = "Turn logging on or off")]
    pub log: bool,
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

        /// Provide multiple variables and their corresponding JSON paths (URL or file).
        ///
        /// You can specify multiple variables by repeating the `--variable` flag,
        /// each specifying a variable name and its corresponding JSON value path.
        #[arg(short, long, value_parser = parse_variable, help = "Variable names and JSON files or URLs")]
        variables: Vec<(String, String)>,
    },
}

fn parse_variable(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() == 2 {
        Ok((parts[0].to_string(), parts[1].to_string()))
    } else {
        Err(format!(
            "Invalid variable format: '{}'. Expected format 'name=path'.",
            s
        ))
    }
}

use std::fs::{self};

use clap::Parser;
use jml_cli::{
    cli::{self, JmlCli},
    util::write_output_to_json,
};
use miette::IntoDiagnostic;

fn main() -> miette::Result<()> {
    let cli = JmlCli::parse();

    match cli.command {
        cli::JmlCommand::Run { file, output } => {
            let source = fs::read_to_string(file).into_diagnostic()?.leak();

            let ast = parser::parse(source).into_diagnostic()?;
            let res = eval::eval_with_source(ast, source)?;

            if let Some(output_path) = output {
                write_output_to_json(output_path, &res)?;
            } else {
                let json = serde_json::to_string_pretty(&res).into_diagnostic()?;
                println!("{}", json);
            }
        }
    }

    Ok(())
}

use std::fs::{self};

use clap::Parser;
use eval::context::Context;
use jml_cli::{
    cli::{self, JmlCli},
    log::setup_logging,
    util::{load_json, write_output_to_json},
};
use miette::IntoDiagnostic;

fn main() -> miette::Result<()> {
    let cli = JmlCli::parse();

    if cli.log {
        setup_logging();
    }

    match cli.command {
        cli::JmlCommand::Run {
            file,
            output,
            variables,
        } => {
            let source = fs::read_to_string(&file).into_diagnostic()?.leak();

            tracing::info!("Processing file: {:?}\n", file);

            let mut ctx = Context::new();

            for (var_name, var_path) in variables {
                tracing::info!("Loading variable '{}' from '{}'\n", var_name, var_path);

                let json_data = load_json(&var_path)?;
                ctx.bind_with_value(var_name.clone(), json_data.clone());

                tracing::info!("Loaded JSON data for '{}'\n", var_name);
            }

            let ast = parser::parse(source).into_diagnostic()?;
            let res = eval::eval_with_ctx_source(ast, source, &mut ctx)?;

            if let Some(output_path) = output {
                tracing::info!("Output will be written to: {:?}", output_path);
                write_output_to_json(output_path, &res)?;
            } else {
                tracing::info!("No output file specified. Printing to console. \n");
                let json = serde_json::to_string_pretty(&res).into_diagnostic()?;
                println!("{}", json);
            }
        }
    }

    Ok(())
}

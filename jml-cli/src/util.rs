use std::{fs::File, path::PathBuf};

use miette::IntoDiagnostic;
use serde::Serialize;

pub fn write_output_to_json<T: Serialize>(
    output_path: PathBuf,
    value: &T,
) -> Result<(), miette::Error> {
    let file = File::create(output_path).into_diagnostic()?;

    serde_json::to_writer_pretty(file, &value).into_diagnostic()?;

    Ok(())
}

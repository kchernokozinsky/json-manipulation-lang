use std::{
    fs::{self, File},
    path::Path,
};

use miette::IntoDiagnostic;
use reqwest::blocking::get;
use serde::Serialize;
use serde_json::Value;

pub fn write_output_to_json<T: Serialize>(
    output_path: impl AsRef<Path>,
    value: &T,
) -> Result<(), miette::Error> {
    let file = File::create(output_path).into_diagnostic()?;

    serde_json::to_writer_pretty(file, &value).into_diagnostic()?;

    Ok(())
}

pub fn load_json(path: &str) -> miette::Result<Value> {
    if path.starts_with("http://") || path.starts_with("https://") {
        let response = get(path).into_diagnostic()?.text().into_diagnostic()?;
        let json: Value = serde_json::from_str(&response).into_diagnostic()?;
        Ok(json)
    } else {
        let data = fs::read_to_string(path).into_diagnostic()?;
        let json: Value = serde_json::from_str(&data).into_diagnostic()?;
        Ok(json)
    }
}

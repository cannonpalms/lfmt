use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};

use anyhow::Result;
use regex::Regex;

#[derive(Debug, clap::Parser)]
pub struct Config {
    /// The key to extract from the logfmt string
    key: String,

    /// Input logfmt string directly as an argument
    input: Option<String>,

    /// Read the logfmt string from a file
    #[clap(short, long)]
    file: Option<PathBuf>,
}

pub fn run(config: Config) -> Result<()> {
    let input = get_input(&config)?;

    let value = extract_value(&input, &config.key)
        .ok_or_else(|| anyhow::anyhow!("Key not found in logfmt string"))?;
    println!("{value}");

    Ok(())
}

/// Parses the logfmt string and extracts the value for the given key
/// Values may contain spaces, in which case the value is quoted
/// The surrounding quotes are not considered part of the value, though.
///
/// Example: extract_value('key="value with spaces"', 'key') -> 'value with spaces'
/// Example: extract_value('key=value', 'key') -> 'value'
/// Example: extract_value('key=value key2="complex value with spaces"', 'key2') -> 'complex value with spaces'
fn extract_value(input: &str, key: &str) -> Option<String> {
    // The regex implementation is disgusting, but it works (for now)
    // Regex to match both quoted and unquoted values
    let re = Regex::new(r#"(\w+)=(?:"([^"]*)"|([^ ]*))"#).unwrap();
    // assign to a variable x to avoid lifetime issues
    let x = re.captures_iter(input).find_map(|caps| {
        if &caps[1] == key {
            Some(if let Some(matched) = caps.get(2) {
                matched.as_str().to_string()
            } else if let Some(matched) = caps.get(3) {
                matched.as_str().to_string()
            } else {
                "".to_string()
            })
        } else {
            None
        }
    });
    x
}

fn get_input(config: &Config) -> Result<String> {
    let input = if let Some(file_path) = config.file.clone() {
        fs::read_to_string(file_path)?
    } else if let Some(input) = config.input.clone() {
        input
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    Ok(input)
}

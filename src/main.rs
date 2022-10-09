use anyhow::{Context, Result};
use clap::Parser;
use log::info;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{self, Write};

#[derive(Parser)]
struct Cli {
    // the pattern to look for
    pattern: String,
    // the path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting up!");

    let args = Cli::parse();
    let path = args.path.to_str().unwrap();

    info!(format!("Reading file {}", &path));
    let f = File::open(&path).with_context(|| format!("could not read file {}", &path))?;
    info!("Successfully read file!");

    let reader = BufReader::new(f);

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    for line in reader.lines() {
        let line = line.with_context(|| "cannot read line")?;

        if line.contains(&args.pattern) {
            writeln!(handle, "{}", &line)?;
        }
    }

    handle.flush()?;
    info!("Done");

    Ok(())
}

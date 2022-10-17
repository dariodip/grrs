use anyhow::{Context, Result};
use clap::Parser;
use log::info;

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

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    info!("Successfully read file!");

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout())?;

    info!("Done");

    Ok(())
}

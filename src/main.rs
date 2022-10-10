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

    find_matches(&content, &args.pattern, &mut std::io::stdout())?;

    info!("Done");

    Ok(())
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).with_context(|| "error while writing to buffer")?;
        }
    }

    Ok(())
}

#[test]
fn find_a_match() {
    use std::vec::Vec;

    let mut buf = Vec::new();
    let result = find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut buf);

    assert!(result.is_ok());
    assert_eq!(buf, b"lorem ipsum\n");
}

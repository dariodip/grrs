use anyhow::{Context, Result};

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
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

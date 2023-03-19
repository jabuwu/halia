use anyhow::Result;
use xshell::{cmd, Shell};

fn main() -> Result<()> {
    let sh = Shell::new()?;
    check(&sh)?;
    Ok(())
}

fn check(sh: &Shell) -> Result<()> {
    cmd!(sh, "cargo check").run()?;
    Ok(())
}

use anyhow::Result;
use xshell::{cmd, Shell};

const HALIA_FEATURES: [&'static str; 1] = ["halia_transform2"];

fn main() -> Result<()> {
    let sh = Shell::new()?;
    check(&sh)?;
    clippy(&sh)?;
    doc_check(&sh)?;
    Ok(())
}

fn check(sh: &Shell) -> Result<()> {
    cmd!(sh, "cargo check --no-default-features").run()?;
    for halia_feature in HALIA_FEATURES {
        cmd!(
            sh,
            "cargo check --no-default-features --features {halia_feature}"
        )
        .run()?;
    }
    cmd!(sh, "cargo check").run()?;
    Ok(())
}

fn clippy(sh: &Shell) -> Result<()> {
    cmd!(sh, "cargo clippy --no-default-features -- -D warnings").run()?;
    for halia_feature in HALIA_FEATURES {
        cmd!(
            sh,
            "cargo clippy --no-default-features --features {halia_feature} -- -D warnings"
        )
        .run()?;
    }
    cmd!(sh, "cargo clippy -- -D warnings").run()?;
    Ok(())
}

fn doc_check(sh: &Shell) -> Result<()> {
    cmd!(
        sh,
        "cargo doc --workspace --all-features --no-deps --document-private-items"
    )
    .run()?;
    Ok(())
}

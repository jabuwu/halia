use std::fs;

use anyhow::Result;
use cargo_toml::Manifest;
use xshell::{cmd, Shell};

fn main() -> Result<()> {
    let cargo = Manifest::from_slice(&fs::read("Cargo.toml").unwrap()).unwrap();
    let halia_features = cargo
        .features
        .iter()
        .map(|(feature, _)| feature.to_owned())
        .filter(|feature| feature.starts_with("halia_"))
        .collect::<Vec<_>>();

    let sh = Shell::new()?;
    clippy(&sh, &halia_features)?;
    check(&sh, &halia_features)?;
    doc_check(&sh)?;
    Ok(())
}

fn clippy(sh: &Shell, halia_features: &Vec<String>) -> Result<()> {
    cmd!(sh, "cargo clippy --no-default-features -- -D warnings").run()?;
    for halia_feature in halia_features {
        cmd!(
            sh,
            "cargo clippy --no-default-features --features {halia_feature} -- -D warnings"
        )
        .run()?;
    }
    cmd!(sh, "cargo clippy -- -D warnings").run()?;
    Ok(())
}

fn check(sh: &Shell, halia_features: &Vec<String>) -> Result<()> {
    cmd!(sh, "cargo check --no-default-features").run()?;
    for halia_feature in halia_features {
        cmd!(
            sh,
            "cargo check --no-default-features --features {halia_feature}"
        )
        .run()?;
    }
    cmd!(sh, "cargo check").run()?;
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

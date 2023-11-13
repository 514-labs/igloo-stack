use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn cannot_run_igloo_init_without_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("igloo-cli")?;

    cmd.arg("init");
    cmd.assert().failure().stderr(predicate::str::contains(
        "the following required arguments were not provided:",
    ));

    Ok(())
}

#[test]
fn can_run_igloo_init() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("igloo-cli")?;

    // TODO add more specific tests when the layout of the
    // app is more stable
    cmd.arg("init")
        .arg("test-app")
        .arg("ts")
        .arg("./tests/test-app");

    cmd.assert().success();

    Ok(())
}
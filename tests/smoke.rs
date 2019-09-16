use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use file_diff::{diff};
use tempfile::NamedTempFile;


#[test]
#[ignore]
fn to_stdout() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dadada")?;
    cmd
        .current_dir("tests/fixtures/occb-exmpl/")
        .arg("--title")
        .arg("My Example")
        .arg("offchaincb.rs")
        .arg("lib.rs");
    cmd.assert()
        .success()
        .stdout(include_str!("fixtures/occb-exmpl/expected.html"));
    Ok(())
}


#[test]
fn can_customise_title() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dadada")?;
    cmd
        .current_dir("tests/fixtures/occb-exmpl/")
        .arg("--title")
        .arg("This is an awesome title")
        .arg("lib.rs");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("<title>This is an awesome title</title>"));

    Ok(())
}

#[test]
fn title_with_unicode_emoji() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dadada")?;
    cmd
        .current_dir("tests/fixtures/occb-exmpl/")
        .arg("--title")
        .arg("title with emoji 🥰")
        .arg("lib.rs");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("<title>title with emoji 🥰</title>"));

    Ok(())
}

#[test]
fn regular_run_works() -> Result<(), Box<dyn std::error::Error>> {
    let file = NamedTempFile::new()?;
    let path = file.path().to_str().unwrap().clone();
    let mut cmd = Command::cargo_bin("dadada")?;
    cmd
        .current_dir("tests/fixtures/occb-exmpl/")
        .arg("--title")
        .arg("My Example")
        .arg("--output")
        .arg(path)
        .arg("offchaincb.rs")
        .arg("lib.rs");
    cmd.assert()
        .success();

    // we check against our local result
    assert!(diff(path, "tests/fixtures/occb-exmpl/expected.html"));

    Ok(())
}

#[test]
fn file_missing() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dadada")?;
    cmd
        .arg("not_existing.rs");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}

use assert_cmd::prelude::*; // Add methods on commands
use file_diff::diff;
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use tempfile::NamedTempFile;

#[test]
#[ignore]
fn to_stdout() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dadada")?;
    cmd.current_dir("tests/fixtures/occb-exmpl/")
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
    cmd.current_dir("tests/fixtures/minimal/")
        .arg("--title")
        .arg("This is an awesome title")
        .arg("lib.rs");
    cmd.assert().success().stdout(predicates::str::contains(
        "<title>This is an awesome title</title>",
    ));

    Ok(())
}

#[test]
fn title_with_unicode_emoji() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dadada")?;
    cmd.current_dir("tests/fixtures/minimal/")
        .arg("--title")
        .arg("title with emoji 🥰")
        .arg("lib.rs");
    cmd.assert().success().stdout(predicates::str::contains(
        "<title>title with emoji 🥰</title>",
    ));

    Ok(())
}

#[test]
fn simple_run() -> Result<(), Box<dyn std::error::Error>> {
    let file = NamedTempFile::new()?;
    let path = file.path().to_str().unwrap().clone();
    let mut cmd = Command::cargo_bin("dadada")?;
    cmd.current_dir("tests/fixtures/minimal/")
        .arg("--title")
        .arg("Minimal Example")
        .arg("--output")
        .arg(path)
        .arg("lib.rs");
    cmd.assert().success();

    // we check against our local result
    assert!(diff(path, "tests/fixtures/minimal/regular.html"));

    Ok(())
}

#[test]
fn customised_run() -> Result<(), Box<dyn std::error::Error>> {
    let file = NamedTempFile::new()?;
    let path = file.path().to_str().unwrap().clone();
    let mut cmd = Command::cargo_bin("dadada")?;
    cmd.current_dir("tests/fixtures/extra_content/")
        .arg("--title")
        .arg("Customised Example")
        .arg("--output")
        .arg(path)
        // here follow our cutom inject
        .arg("--meta")
        .arg("meta.html")
        .arg("--header")
        .arg("header.html")
        .arg("--footer")
        .arg("footer.md")
        .arg("lib.rs");
    cmd.assert().success();

    // we check against our local result
    assert!(diff(path, "tests/fixtures/extra_content/expected.html"));

    Ok(())
}

#[test]
fn no_css() -> Result<(), Box<dyn std::error::Error>> {
    let file = NamedTempFile::new()?;
    let path = file.path().to_str().unwrap().clone();
    let mut cmd = Command::cargo_bin("dadada")?;
    cmd.current_dir("tests/fixtures/minimal/")
        .arg("--title")
        .arg("Minimal Example")
        .arg("--output")
        .arg(path)
        .arg("--no-css")
        .arg("lib.rs");
    cmd.assert().success();

    // we check against our local result
    assert!(diff(path, "tests/fixtures/minimal/no-css.html"));

    Ok(())
}

#[test]
fn no_js() -> Result<(), Box<dyn std::error::Error>> {
    let file = NamedTempFile::new()?;
    let path = file.path().to_str().unwrap().clone();
    let mut cmd = Command::cargo_bin("dadada")?;
    cmd.current_dir("tests/fixtures/minimal/")
        .arg("--title")
        .arg("Minimal Example")
        .arg("--output")
        .arg(path)
        .arg("--no-js")
        .arg("lib.rs");
    cmd.assert().success();

    // we check against our local result
    assert!(diff(path, "tests/fixtures/minimal/no-js.html"));

    Ok(())
}

#[test]
fn no_css_nor_js() -> Result<(), Box<dyn std::error::Error>> {
    let file = NamedTempFile::new()?;
    let path = file.path().to_str().unwrap().clone();
    let mut cmd = Command::cargo_bin("dadada")?;
    cmd.current_dir("tests/fixtures/minimal/")
        .arg("--title")
        .arg("Minimal Example")
        .arg("--output")
        .arg(path)
        .arg("--no-css")
        .arg("--no-js")
        .arg("lib.rs");
    cmd.assert().success();

    // we check against our local result
    assert!(diff(path, "tests/fixtures/minimal/neither.html"));

    Ok(())
}

#[test]
fn regular_big_run_works() -> Result<(), Box<dyn std::error::Error>> {
    let file = NamedTempFile::new()?;
    let path = file.path().to_str().unwrap().clone();
    let mut cmd = Command::cargo_bin("dadada")?;
    cmd.current_dir("tests/fixtures/occb-exmpl/")
        .arg("--title")
        .arg("My Example")
        .arg("--output")
        .arg(path)
        .arg("offchaincb.rs")
        .arg("lib.rs");
    cmd.assert().success();

    // we check against our local result
    assert!(diff(path, "tests/fixtures/occb-exmpl/expected.html"));

    Ok(())
}

#[test]
fn file_missing() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dadada")?;
    cmd.arg("not_existing.rs");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}

use assert_cmd::{Command, output::OutputOkExt};
use predicates::prelude::*;
use std::{fs, vec, ffi::{OsStr, OsString}};

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
  Command::cargo_bin("echor")?
    .assert()
    .failure()
    .stderr(predicate::str::contains("USAGE"));
  Ok(())
}

#[test]
fn hello1() -> TestResult {
  let outfile = "tests/expected/hello1.txt";
  let expected = fs::read_to_string(outfile)?;
  let mut cmd = Command::cargo_bin("echor")?;
  cmd.arg("Hello there").assert().success().stdout(expected + "\n");
  Ok(())
}

#[test]
fn hello2() -> TestResult {
  let expected = fs::read_to_string("tests/expected/hello2.txt")?;
  let mut cmd = Command::cargo_bin("echor")?;
  cmd.args(vec!["Hello", "there"])
    .assert()
    .success()
    .stdout(expected);
  Ok(())
}


fn run(args: &[&str], expected_file: &str) -> TestResult {
  let expected = fs::read_to_string(expected_file)?;
  Command::cargo_bin("echor")?
    .args(args)
    .assert()
    .success()
    .stdout(expected);
  Ok(())
}

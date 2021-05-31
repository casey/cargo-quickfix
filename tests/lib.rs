use std::{
  fs::{self, OpenOptions},
  io::Write,
  process::Command,
  str,
};

use test_context::TestContext;

mod test_context;

#[track_caller]
fn assert_contains(haystack: &str, needle: &str) {
  assert!(
    haystack.contains(needle),
    "\n{:?} does not contain {:?}\n",
    haystack,
    needle
  );
}

#[test]
fn execute_cargo_command() {
  let context = TestContext::new();

  let output = context.quickfix_command().arg("build").output().unwrap();

  assert!(output.status.success());

  let output = Command::new(context.project_binary()).output().unwrap();

  assert!(output.status.success());

  assert_eq!(str::from_utf8(&output.stdout).unwrap(), "Hello, world!\n");
}

#[test]
fn propagate_command_status() {
  let context = TestContext::new();

  let output = context.quickfix_command().arg("asdf").output().unwrap();

  assert!(!output.status.success());
}

#[test]
fn propagate_command_stderr() {
  let context = TestContext::new();

  let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(context.project().join("src").join("main.rs"))
    .unwrap();

  writeln!(file, "BAD").unwrap();

  let output = context.quickfix_command().arg("build").output().unwrap();

  assert!(!output.status.success());

  let stderr = str::from_utf8(&output.stderr).unwrap();

  assert_contains(stderr, "Compiling\u{1b}[0m project");
  assert_contains(stderr, "error\u{1b}[0m\u{1b}[0m\u{1b}");
}

#[test]
fn populate_errorfile() {
  let context = TestContext::new();

  let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(context.project().join("src").join("main.rs"))
    .unwrap();

  writeln!(file, "BAD").unwrap();

  let output = context.quickfix_command().arg("build").output().unwrap();

  assert!(!output.status.success());

  let errorfile = fs::read_to_string(context.project().join(".errors.txt")).unwrap();

  assert_contains(&errorfile, "Compiling project");
  assert_contains(&errorfile, "error: expected one of");
}

#[test]
fn strip_escape_sequences() {
  let context = TestContext::new();

  let output = context
    .quickfix_command()
    .args(&["build"])
    .output()
    .unwrap();

  assert!(output.status.success());

  let errorfile = fs::read_to_string(context.project().join(".errors.txt")).unwrap();

  assert_contains(&errorfile, "Compiling project");
}

#[test]
fn default_to_check() {
  let context = TestContext::new();

  let output = context.quickfix_command().output().unwrap();

  assert!(output.status.success());

  let stderr = str::from_utf8(&output.stderr).unwrap();

  assert_contains(&stderr, "Checking\u{1b}[0m project");
}

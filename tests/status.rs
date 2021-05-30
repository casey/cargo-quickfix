use crate::common::*;

#[test]
fn help_returns_success() {
  assert!(Command::new(executable_path("quickfix"))
    .arg("--help")
    .output()
    .unwrap()
    .status
    .success());
}

#[test]
fn version_returns_success() {
  assert!(Command::new(executable_path("quickfix"))
    .arg("--version")
    .output()
    .unwrap()
    .status
    .success());
}

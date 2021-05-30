use crate::common::*;

#[test]
fn print_help_to_stdout() {
  assert!(str::from_utf8(
    &Command::new(executable_path("quickfix"))
      .arg("--help")
      .output()
      .unwrap()
      .stdout
  )
  .unwrap()
  .starts_with("quickfix 0.0.0\n\nUSAGE:\n"));
}

#[test]
fn print_version_to_stdout() {
  assert_eq!(
    str::from_utf8(
      &Command::new(executable_path("quickfix"))
        .arg("--version")
        .output()
        .unwrap()
        .stdout
    )
    .unwrap(),
    "quickfix 0.0.0\n"
  );
}

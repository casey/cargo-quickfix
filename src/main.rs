use crate::common::*;

mod common;
mod environment;
mod format_string;
mod formatted_message;
mod output;
mod output_stream;
mod subcommand;
mod token;

fn main() {
  Environment::main().run();
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::process::Command;

  #[test]
  fn create_quickfix_file() {
    let output = Environment::test().set_args(&["write"]).output();

    assert!(output.dir().join(".errorfile").is_file());
  }

  #[test]
  fn populate_quickfix_file_with_errors() {
    let tempdir = tempfile::tempdir().unwrap();

    let output = Command::new("cargo")
      .args(&["init", "--name", "foo"])
      .arg(tempdir.path())
      .output()
      .unwrap();

    assert!(output.status.success());

    fs::write(
      tempdir.path().join("src/main.rs"),
      "fn main() {\n  let foo foo;\n}\n",
    )
    .unwrap();

    let output = Command::new("cargo")
      .args(&["check", "--message-format=json"])
      .current_dir(tempdir.path())
      .output()
      .unwrap();

    assert!(!output.status.success());

    let errors = str::from_utf8(&output.stdout).unwrap();

    let output = Environment::test()
      .set_args(&["write"])
      .set_stdin(&errors)
      .output();

    let errors = fs::read_to_string(output.dir().join(".errorfile")).unwrap();

    assert_eq!(
      errors,
      "\
file: src/main.rs
line: 2
column: 11
message: expected one of `:`, `;`, `=`, `@`, or `|`, found `foo`
---
"
    );
  }

  #[test]
  fn print_errorformat() {
    let output = Environment::test().set_args(&["errorformat"]).output();

    assert_eq!(
      output.stdout(),
      "%Efile:\\ %f,%Cline:\\ %l,%Ccolumn:\\ %c,%Cmessage:\\ %m,%Z---"
    );
  }
}

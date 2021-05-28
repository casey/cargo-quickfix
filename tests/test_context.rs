use executable_path::executable_path;
use std::{
  env,
  ffi::OsString,
  fs, iter,
  path::{Path, PathBuf},
  process::Command,
};
use tempfile::{tempdir, TempDir};

pub(crate) struct TestContext {
  #[allow(unused)]
  bin:            TempDir,
  #[allow(unused)]
  dir:            TempDir,
  project:        PathBuf,
  project_binary: PathBuf,
  path:           OsString,
}

impl TestContext {
  pub(crate) fn new() -> Self {
    let dir = tempdir().unwrap();

    let output = Command::new("cargo")
      .arg("init")
      .arg("project")
      .current_dir(&dir.path())
      .output()
      .unwrap();

    assert!(output.status.success());

    let project = dir.path().join("project");

    let bin = tempdir().unwrap();

    fs::copy(
      executable_path("cargo-quickfix"),
      bin.path().join("cargo-quickfix-test"),
    )
    .unwrap();

    let path = env::join_paths(
      iter::once(bin.path().to_owned()).chain(env::split_paths(&env::var_os("PATH").unwrap())),
    )
    .unwrap();

    let project_binary = project.join("target").join("debug").join("project");

    Self {
      dir,
      project,
      bin,
      path,
      project_binary,
    }
  }

  pub(crate) fn project(&self) -> &Path {
    &self.project
  }

  pub(crate) fn cargo_command(&self) -> Command {
    let mut command = Command::new("cargo");

    command.current_dir(self.project()).env("PATH", &self.path);

    command
  }

  pub(crate) fn quickfix_command(&self) -> Command {
    let mut command = self.cargo_command();

    command.arg("quickfix-test");

    command
  }

  pub(crate) fn project_binary(&self) -> &Path {
    &self.project_binary
  }
}

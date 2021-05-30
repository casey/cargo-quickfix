use crate::common::*;

pub(crate) struct Output {
  pub(crate) stdout: String,
  pub(crate) stderr: String,
  pub(crate) dir:    Box<dyn AsRef<Path>>,
}

impl Output {
  pub(crate) fn stdout(&self) -> &str {
    &self.stdout
  }

  pub(crate) fn dir(&self) -> &Path {
    self.dir.as_ref().as_ref()
  }
}

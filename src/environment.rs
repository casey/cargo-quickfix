use crate::common::*;

pub(crate) struct Environment {
  args:        Vec<OsString>,
  current_dir: Box<dyn AsRef<Path>>,
  stdin:       Box<dyn Read>,
  stdout:      Box<dyn OutputStream>,
}

impl Environment {
  pub(crate) fn main() -> Self {
    Self {
      args:        env::args_os().into_iter().collect(),
      current_dir: Box::new(env::current_dir().unwrap()),
      stdin:       Box::new(io::stdin()),
      stdout:      Box::new(io::stdout()),
    }
  }

  #[cfg(test)]
  pub(crate) fn test() -> Self {
    Environment {
      args:        Vec::new(),
      current_dir: Box::new(tempfile::tempdir().unwrap()),
      stdin:       Box::new(Cursor::new(Vec::new())),
      stdout:      Box::new(Cursor::new(Vec::new())),
    }
  }

  pub(crate) fn stdout(&mut self) -> &mut dyn OutputStream {
    self.stdout.as_mut()
  }

  pub(crate) fn stdin(&mut self) -> &mut dyn Read {
    self.stdin.as_mut()
  }

  #[cfg(test)]
  pub(crate) fn set_args(mut self, args: &[&str]) -> Self {
    self.args = iter::once("quickfix")
      .chain(args.iter().cloned())
      .map(OsString::from)
      .collect();

    self
  }

  #[cfg(test)]
  pub(crate) fn set_stdin(mut self, stdin: &str) -> Self {
    self.stdin = Box::new(Cursor::new(stdin.as_bytes().to_owned()));

    self
  }

  pub(crate) fn current_dir(&self) -> &Path {
    self.current_dir.as_ref().as_ref()
  }

  pub(crate) fn run(mut self) {
    match Subcommand::from_iter_safe(&self.args) {
      Ok(subcommand) => subcommand.run(&mut self),
      Err(error) => error.exit(),
    }
  }

  #[cfg(test)]
  pub(crate) fn output(mut self) -> Output {
    Subcommand::from_iter_safe(&self.args)
      .unwrap()
      .run(&mut self);

    Output {
      dir:    self.current_dir,
      stdout: str::from_utf8(self.stdout.as_ref().capture())
        .unwrap()
        .to_owned(),
    }
  }
}

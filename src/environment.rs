use crate::common::*;

pub(crate) struct Environment {
  args:        Vec<OsString>,
  current_dir: Box<dyn AsRef<Path>>,
  stdin:       Box<dyn Read>,
  stdout:      Box<dyn OutputStream>,
  stderr:      Box<dyn OutputStream>,
}

impl Environment {
  pub(crate) fn production() -> Result<Self> {
    Ok(Self {
      args:        env::args_os().into_iter().collect(),
      current_dir: Box::new(env::current_dir().context(error::CurrentDir)?),
      stdin:       Box::new(io::stdin()),
      stdout:      Box::new(io::stdout()),
      stderr:      Box::new(io::stderr()),
    })
  }

  #[cfg(test)]
  pub(crate) fn test() -> Self {
    Environment {
      args:        Vec::new(),
      current_dir: Box::new(tempfile::tempdir().unwrap()),
      stdin:       Box::new(Cursor::new(Vec::new())),
      stdout:      Box::new(Cursor::new(Vec::new())),
      stderr:      Box::new(Cursor::new(Vec::new())),
    }
  }

  pub(crate) fn main() -> Result<(), i32> {
    Self::production().print()?.run()
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

  pub(crate) fn run(&mut self) -> Result<(), i32> {
    match Subcommand::from_iter_safe(&self.args).context(error::Arguments) {
      Ok(subcommand) => subcommand.run(self).print(),
      Err(Error::Arguments { source }) if !source.use_stderr() => {
        if let Err(error) = write!(self.stdout, "{}", source) {
          write!(self.stderr, "error: Failed to write to stdout: {}", error).ok();
        }
        Ok(())
      },
      Err(error) => {
        write!(self.stderr, "error: {}", error).ok();
        Err(error.code())
      },
    }
  }

  #[cfg(test)]
  fn output(self) -> Output {
    Output {
      dir:    self.current_dir,
      stdout: str::from_utf8(self.stdout.as_ref().capture())
        .unwrap()
        .to_owned(),
      stderr: str::from_utf8(self.stderr.as_ref().capture())
        .unwrap()
        .to_owned(),
    }
  }

  #[cfg(test)]
  pub(crate) fn ok(mut self) -> Output {
    self.run().unwrap();
    self.output()
  }

  #[cfg(test)]
  pub(crate) fn err(mut self) -> Output {
    self.run().unwrap_err();
    self.output()
  }
}

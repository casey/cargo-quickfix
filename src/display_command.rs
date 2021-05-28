use crate::common::*;

pub(crate) struct DisplayCommand<'a> {
  command:   &'a OsStr,
  arguments: &'a [OsString],
}

impl<'a> DisplayCommand<'a> {
  pub(crate) fn new(command: &'a OsStr, arguments: &'a [OsString]) -> Self {
    DisplayCommand { command, arguments }
  }
}

impl<'a> Display for DisplayCommand<'a> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "`{}`", self.command.to_string_lossy())?;
    for argument in self.arguments {
      write!(f, ", `{}`", argument.to_string_lossy())?;
    }
    Ok(())
  }
}

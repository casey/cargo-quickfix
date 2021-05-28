use crate::common::*;

#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
pub(crate) enum Error {
  #[snafu(display(
    "Failed to invoke command {}: {}",
    DisplayCommand::new(&command, &arguments),
    source
  ))]
  Spawn {
    command:   OsString,
    arguments: Vec<OsString>,
    source:    io::Error,
  },
  #[snafu(display(
    "Failed to wait for command {}: {}",
    DisplayCommand::new(&command, &arguments),
    source
  ))]
  Wait {
    command:   OsString,
    arguments: Vec<OsString>,
    source:    io::Error,
  },
  #[snafu(display(
    "Command {} failed: {}",
    DisplayCommand::new(&command, &arguments),
    status,
  ))]
  Status {
    command:   OsString,
    arguments: Vec<OsString>,
    status:    ExitStatus,
  },
  #[snafu(display("I/O error at `{}`: {}", path.display(), source))]
  Filesystem { path: PathBuf, source: io::Error },
  #[snafu(display(
    "I/O copying from cargo command standard error to errorfile `{}`: {}",
    path.display(),
    source,
  ))]
  Copy { path: PathBuf, source: io::Error },
}

impl Error {
  pub(crate) fn display(&self) -> bool {
    match self {
      Self::Filesystem { .. } | Self::Spawn { .. } | Self::Wait { .. } | Self::Copy { .. } => true,
      Self::Status { .. } => false,
    }
  }

  pub(crate) fn code(&self) -> i32 {
    match self {
      Self::Status { status, .. } => status.code().unwrap_or(libc::EXIT_FAILURE),
      _ => libc::EXIT_FAILURE,
    }
  }
}

use crate::common::*;

#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
pub(crate) enum Error {
  #[snafu(display("Failed to retrieve current directory: {}", source))]
  CurrentDir { source: io::Error },
  #[snafu(display("{}", source))]
  Arguments { source: structopt::clap::Error },
  #[snafu(display("I/O error at `{}`: {}", path.display(), source))]
  Filesystem { source: io::Error, path: PathBuf },
  #[snafu(display("Failed to read from stdin: {}", source))]
  Stdin { source: io::Error },
  #[snafu(display("Failed to write to stdout: {}", source))]
  Stdout { source: io::Error },
  #[snafu(display("Failed to write to stderr: {}", source))]
  Stderr { source: io::Error },
}

impl Error {
  pub(crate) fn code(self) -> i32 {
    libc::EXIT_FAILURE
  }
}

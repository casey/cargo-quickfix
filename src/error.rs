use crate::common::*;

#[derive(Snafu, Debug)]
pub(crate) enum Error {}

impl Error {
  pub(crate) fn code(self) -> i32 {
    libc::EXIT_FAILURE
  }
}

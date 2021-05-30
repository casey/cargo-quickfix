use crate::common::*;

pub(crate) trait ResultExt<T> {
  fn print(self) -> Result<T, i32>;
}

impl<T> ResultExt<T> for Result<T, Error> {
  fn print(self) -> Result<T, i32> {
    match self {
      Ok(ok) => Ok(ok),
      Err(error) => {
        eprintln!("error: {}", error);
        Err(error.code())
      },
    }
  }
}

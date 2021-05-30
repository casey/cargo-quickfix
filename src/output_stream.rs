use crate::common::*;

pub(crate) trait OutputStream: Write {
  #[cfg(test)]
  fn capture(&self) -> &[u8];

  #[cfg(test)]
  fn cursor(&self) -> Cursor<Vec<u8>>;
}

impl OutputStream for Stdout {
  #[cfg(test)]
  fn capture(&self) -> &[u8] {
    panic!("Cannot call OutputStream::capture on Stdout");
  }

  #[cfg(test)]
  fn cursor(&self) -> Cursor<Vec<u8>> {
    panic!("Cannot call OutputStream::cursor on Stdout");
  }
}

impl OutputStream for Stderr {
  #[cfg(test)]
  fn capture(&self) -> &[u8] {
    panic!("Cannot call OutputStream::capture on Stdout");
  }

  #[cfg(test)]
  fn cursor(&self) -> Cursor<Vec<u8>> {
    panic!("Cannot call OutputStream::cursor on Stdout");
  }
}

impl OutputStream for Cursor<Vec<u8>> {
  #[cfg(test)]
  fn capture(&self) -> &[u8] {
    self.get_ref()
  }

  #[cfg(test)]
  fn cursor(&self) -> Cursor<Vec<u8>> {
    self.clone()
  }
}

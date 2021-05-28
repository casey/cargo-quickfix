use crate::common::*;

pub(crate) trait OutputStream: Write {
  #[cfg(test)]
  fn capture(&self) -> &[u8];
}

impl OutputStream for Stdout {
  #[cfg(test)]
  fn capture(&self) -> &[u8] {
    panic!("Cannot call OutpuStream::capture on Stdout");
  }
}

impl OutputStream for Cursor<Vec<u8>> {
  #[cfg(test)]
  fn capture(&self) -> &[u8] {
    self.get_ref()
  }
}

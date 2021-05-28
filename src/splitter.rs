use crate::common::*;

pub(crate) struct Splitter<A: Write, B: Write>(A, B);

impl<A: Write, B: Write> Splitter<A, B> {
  pub(crate) fn new(a: A, b: B) -> Self {
    Self(a, b)
  }
}

impl<A: Write, B: Write> Write for Splitter<A, B> {
  fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
    self.0.write_all(bytes)?;
    self.1.write_all(bytes)?;
    Ok(bytes.len())
  }

  fn flush(&mut self) -> io::Result<()> {
    self.0.flush()?;
    self.1.flush()?;
    Ok(())
  }
}

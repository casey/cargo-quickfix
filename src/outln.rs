macro_rules! outln {
  ($environment:expr) => {
    writeln!($environment.stdout(), "").context(crate::error::Stdout)
  };
  ($environment:expr, $fmt:expr) => {
    writeln!($environment.stdout(), $fmt).context(crate::error::Stdout)
  };
  ($environment:expr, $fmt:expr, $($arg:tt)*) => {
    writeln!($environment.stdout(), $fmt, $($arg)*).context(crate::error::Stdout)
  };
}

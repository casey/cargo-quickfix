pub(crate) use std::{
  env,
  ffi::OsString,
  fmt::{self, Display, Formatter},
  fs::File,
  io::{self, BufReader, Cursor, Read, Stdout, Write},
  path::Path,
  process, str,
};

pub(crate) use ::{
  cargo_metadata::{
    diagnostic::{DiagnosticLevel, DiagnosticSpan},
    Message,
  },
  snafu::Snafu,
  structopt::StructOpt,
};

pub(crate) use crate::{
  environment::Environment, error::Error, format_string::FormatString,
  formatted_message::FormattedMessage, output_stream::OutputStream, result::Result,
  subcommand::Subcommand, token::Token,
};

#[cfg(test)]
mod test {
  pub(crate) use std::{fs, iter, path};

  pub(crate) use crate::output::Output;
}

#[cfg(test)]
pub(crate) use test::*;

pub(crate) use std::{
  env,
  ffi::OsString,
  fmt::{self, Display, Formatter},
  fs::File,
  io::{self, BufReader, Cursor, Read, Stderr, Stdout, Write},
  path::{Path, PathBuf},
  process, str,
};

pub(crate) use ::{
  cargo_metadata::{
    diagnostic::{DiagnosticLevel, DiagnosticSpan},
    Message,
  },
  snafu::{ResultExt as _, Snafu},
  structopt::StructOpt,
};

pub(crate) use crate::error;

pub(crate) use crate::{output_stream::OutputStream, result_ext::ResultExt};

pub(crate) use crate::{
  environment::Environment, error::Error, format_string::FormatString,
  formatted_message::FormattedMessage, result::Result, subcommand::Subcommand, token::Token,
};

#[cfg(test)]
mod test {
  pub(crate) use std::{fs, iter, path};

  pub(crate) use crate::output::Output;
}

#[cfg(test)]
pub(crate) use test::*;

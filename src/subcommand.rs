use crate::common::*;

#[derive(StructOpt)]
pub(crate) enum Subcommand {
  Write,
  Errorformat,
}

impl Subcommand {
  const ERROR: FormatString = FormatString::new(&[
    Token::Text("file: "),
    Token::File,
    Token::Text("line: "),
    Token::Line,
    Token::Text("column: "),
    Token::Column,
    Token::Text("message: "),
    Token::Message,
    Token::Text("---"),
  ]);

  pub(crate) fn run(self, environment: &mut Environment) -> Result<()> {
    match self {
      Self::Write => Self::write(environment),
      Self::Errorformat => Self::errorformat(environment),
    }
  }

  fn write(environment: &mut Environment) -> Result<()> {
    let path = environment.current_dir().join(".errorfile");

    let mut errorfile = File::create(&path).context(error::Filesystem { path: &path })?;

    let (stdin, _stdout, stderr) = environment.standard_streams();

    let reader = BufReader::new(stdin);

    for result in Message::parse_stream(reader) {
      let message = result.context(error::Stdin)?;
      match message {
        Message::CompilerMessage(message) => {
          if let Some(rendered) = message.message.rendered {
            writeln!(stderr, "{}", rendered).context(error::Stderr)?;
          }
          if message.message.level == DiagnosticLevel::Error {
            if let Some(span) = message.message.spans.iter().find(|span| span.is_primary) {
              writeln!(
                errorfile,
                "{}",
                Self::ERROR.format(span, &message.message.message)
              )
              .context(error::Filesystem { path: &path })?;
              errorfile
                .flush()
                .context(error::Filesystem { path: &path })?;
            }
          }
        },
        _ => {},
      }
    }

    Ok(())
  }

  fn errorformat(environment: &mut Environment) -> Result<()> {
    outln!(environment, "{}", Self::ERROR)?;
    Ok(())
  }
}

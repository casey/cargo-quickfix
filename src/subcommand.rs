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
    let mut errorfile = File::create(environment.current_dir().join(".errorfile")).unwrap();

    let reader = BufReader::new(environment.stdin());

    for message in Message::parse_stream(reader) {
      match message.unwrap() {
        Message::BuildFinished(finished) => {
          println!("{:?}", finished);
        },
        Message::BuildScriptExecuted(script) => {
          println!("{:?}", script);
        },
        Message::CompilerArtifact(artifact) => {
          println!("{:?}", artifact);
        },
        Message::CompilerMessage(message) => {
          println!("{:?}", message);
          if message.message.level == DiagnosticLevel::Error {
            if let Some(span) = message.message.spans.iter().find(|span| span.is_primary) {
              writeln!(
                errorfile,
                "{}",
                Self::ERROR.format(span, &message.message.message)
              )
              .unwrap();
              errorfile.flush().unwrap();
            }
          }
        },
        Message::TextLine(line) => {
          println!("{}", line);
        },
        _ => (),
      }
    }

    Ok(())
  }

  fn errorformat(environment: &mut Environment) -> Result<()> {
    write!(environment.stdout(), "{}", Self::ERROR).unwrap();

    Ok(())
  }
}

use crate::common::*;

pub(crate) struct FormatString {
  tokens: &'static [Token],
}

impl FormatString {
  pub(crate) const fn new(tokens: &'static [Token]) -> Self {
    Self { tokens }
  }

  pub(crate) fn format<'a>(
    &'a self,
    span: &'a DiagnosticSpan,
    message: &'a str,
  ) -> FormattedMessage<'a> {
    FormattedMessage::new(self.tokens, span, message)
  }
}

impl Display for FormatString {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let mut last = None;

    for (i, token) in self.tokens.iter().enumerate() {
      match i {
        0 => write!(f, "%E")?,
        i if i == self.tokens.len() - 1 => write!(f, "%Z")?,
        _ =>
          if !matches!(last, Some(Token::Text(_))) {
            write!(f, "%C")?;
          },
      }

      match token {
        Token::Column => write!(f, "%c,")?,
        Token::File => write!(f, "%f,")?,
        Token::Line => write!(f, "%l,")?,
        Token::Message => write!(f, "%m,")?,
        Token::Text(text) =>
          for c in text.chars() {
            match c {
              ' ' => write!(f, "\\ ")?,
              c => write!(f, "{}", c)?,
            }
          },
      }

      last = Some(*token)
    }
    Ok(())
  }
}

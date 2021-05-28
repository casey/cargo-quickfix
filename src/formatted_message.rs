use crate::common::*;

pub(crate) struct FormattedMessage<'a> {
  tokens:  &'a [Token],
  span:    &'a DiagnosticSpan,
  message: &'a str,
}

impl<'a> FormattedMessage<'a> {
  pub(crate) fn new(tokens: &'a [Token], span: &'a DiagnosticSpan, message: &'a str) -> Self {
    FormattedMessage {
      tokens,
      span,
      message,
    }
  }
}

impl<'a> Display for FormattedMessage<'a> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    for token in self.tokens {
      match token {
        Token::Column => writeln!(f, "{}", self.span.column_start)?,
        Token::File => writeln!(f, "{}", self.span.file_name)?,
        Token::Line => writeln!(f, "{}", self.span.line_start)?,
        Token::Message => writeln!(f, "{}", self.message)?,
        Token::Text(text) => write!(f, "{}", text)?,
      }
    }

    Ok(())
  }
}

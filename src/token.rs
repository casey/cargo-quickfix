use crate::common::*;

#[derive(Copy, Clone)]
pub(crate) enum Token {
  Column,
  File,
  Line,
  Message,
  Text(&'static str),
}

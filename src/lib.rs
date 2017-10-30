#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

pub enum Token {
  Ampersand,       // &
  AmpersandDouble, // &&
  AngleDoubleR,    // >>
  AngleL,          // <
  AngleR,          // >
  Pipe,            // |
  PipeDouble,      // ||
  Semicolon,       // ;
  Word,            // 'hello' '/bob/frank' '-u' '-'
}

fn re(pattern: &str) -> Regex {
  Regex::new(pattern).unwrap()
}

impl Token {
  pub fn regex(self) -> &'static Regex {
    lazy_static! {
      static ref AMPERSAND:        Regex = re(r"&");
      static ref AMPERSAND_DOUBLE: Regex = re(r"&&");
      static ref ANGLE_DOUBLE_R:   Regex = re(r">>");
      static ref ANGLE_L:          Regex = re(r"<");
      static ref ANGLE_R:          Regex = re(r">");
      static ref PIPE:             Regex = re(r"\|");
      static ref PIPE_DOUBLE:      Regex = re(r"\|\|");
      static ref SEMICOLON:        Regex = re(r";");
      static ref WORD:             Regex = re(r"[a-zA-Z0-9_/-]+");
    }

    match self {
      Token::Ampersand       => &AMPERSAND,
      Token::AmpersandDouble => &AMPERSAND_DOUBLE,
      Token::AngleDoubleR    => &ANGLE_DOUBLE_R,
      Token::AngleL          => &ANGLE_L,
      Token::AngleR          => &ANGLE_R,
      Token::Pipe            => &PIPE,
      Token::PipeDouble      => &PIPE_DOUBLE,
      Token::Semicolon       => &SEMICOLON,
      Token::Word            => &WORD,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn regex() {
    assert_eq!(Token::AmpersandDouble.regex().as_str(), "&&")
  }
}

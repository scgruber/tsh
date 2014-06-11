use std::char;
use std::fmt;
use cmd;

/// States of the DFA.
enum State {
  INITIAL,
  Q,
  QU,
  QUI,
  QUIT,
  J,
  JO,
  JOB,
  JOBS,
  F,
  FG,
  FGN,
  B,
  BG,
  BGN
}

impl PartialEq for State {
  fn eq(&self, other: &State) -> bool {
    (*self as int) == (*other as int)
  }
}

impl fmt::Show for State {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      INITIAL => write!(f, "INITIAL"),
      Q => write!(f, "Q"),
      QU => write!(f, "QU"),
      QUI => write!(f, "QUI"),
      QUIT => write!(f, "QUIT"),
      J => write!(f, "J"),
      JO => write!(f, "JO"),
      JOB => write!(f, "JOB"),
      JOBS => write!(f, "JOBS"),
      F => write!(f, "F"),
      FG => write!(f, "FG"),
      FGN => write!(f, "FGN"),
      B => write!(f, "B"),
      BG => write!(f, "BG"),
      BGN => write!(f, "BGN")
    }
  }
}

/// Matches the final states of the DFA to the built-in commands. 
fn state_to_builtin(state: State, jid: int) -> Option<cmd::BuiltinCmd> {
  match state {
    QUIT => Some(cmd::Quit),
    JOBS => Some(cmd::Jobs),
    FGN => Some(cmd::Foreground),
    BGN => Some(cmd::Background),
    _ => None
  }
}

/// Processes the input string through a DFA to determine if any built-in 
/// commands are being activated by the user.
pub fn builtin_parse_dfa(input: &str) -> Option<cmd::BuiltinCmd> {
  let mut dfa_state = INITIAL;
  let mut jid : int = -1;
  for character in input.chars() {
    match character {
      'b' if dfa_state == INITIAL => dfa_state = B,
      'b' if dfa_state == JO => dfa_state = JOB,
      'f' if dfa_state == INITIAL => dfa_state = F,
      'g' if dfa_state == F => dfa_state = FG,
      'g' if dfa_state == B => dfa_state = BG,
      'i' if dfa_state == QU => dfa_state = QUI,
      'j' if dfa_state == INITIAL => dfa_state = J,
      'o' if dfa_state == J => dfa_state = JO,
      'q' if dfa_state == INITIAL => dfa_state = Q,
      's' if dfa_state == JOB => dfa_state = JOBS,
      't' if dfa_state == QUI => dfa_state = QUIT,
      'u' if dfa_state == Q => dfa_state = QU,
      ' ' | '\t' | '\n' => (),
      ch if char::is_digit(ch) => {
        match dfa_state {
          FG => {
            dfa_state = FGN;
            jid = char::to_digit(ch,10).unwrap() as int;
          },
          FGN => {
            jid *= 10;
            jid += char::to_digit(ch,10).unwrap() as int;
          },
          BG => {
            dfa_state = BGN;
            jid = char::to_digit(ch,10).unwrap() as int;
          }
          BGN => {
            jid *= 10;
            jid += char::to_digit(ch,10).unwrap() as int;
          },
          _ => return None
        }
      }
      _ => return None
    }
  };
  state_to_builtin(dfa_state, jid)
}

#[test]
fn builtin_parse_dfa_quit() {
  let input = "quit";
  match builtin_parse_dfa(input) {
    Some(cmd::Quit) => (),
    _ => fail!()
  }
}

#[test]
fn builtin_parse_dfa_jobs() {
  let input = "jobs";
  match builtin_parse_dfa(input) {
    Some(cmd::Jobs) => (),
    _ => fail!()
  }
}

#[test]
fn builtin_parse_dfa_fg() {
  let input = "fg 1234";
  match builtin_parse_dfa(input) {
    Some(cmd::Foreground) => (),
    _ => fail!()
  }
}

#[test]
fn builtin_parse_dfa_bg() {
  let input = "bg 02846";
  match builtin_parse_dfa(input) {
    Some(cmd::Background) => (),
    _ => fail!()
  }
}

#[test]
fn builtin_parse_dfa_whitespace() {
  let input = "  \t ";
  match builtin_parse_dfa(input) {
    None => (),
    _ => fail!()
  }
}
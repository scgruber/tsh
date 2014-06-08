use std::char;
use cmd;

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

fn state_to_builtin(state: State, jid: int) -> Option<cmd::BuiltinCmd> {
  match state {
    QUIT => Some(cmd::Quit),
    JOBS => Some(cmd::Jobs),
    FGN => Some(cmd::Foreground),
    BGN => Some(cmd::Background),
    _ => None
  }
}

pub fn builtin_parse_dfa(input: Vec<u16>) -> Option<cmd::BuiltinCmd> {
  let mut dfa_state = INITIAL;
  let mut jid : int = -1;
  for current_character in input.iter() {
    let character = char::from_u32(*current_character as u32);
    match character {
      None => return state_to_builtin(dfa_state, jid),
      Some('b') if dfa_state == INITIAL => dfa_state = B,
      Some('b') if dfa_state == JO => dfa_state = JOB,
      Some('f') if dfa_state == INITIAL => dfa_state = F,
      Some('g') if dfa_state == F => dfa_state = FG,
      Some('g') if dfa_state == B => dfa_state = BG,
      Some('i') if dfa_state == QU => dfa_state = QUI,
      Some('j') if dfa_state == INITIAL => dfa_state = J,
      Some('o') if dfa_state == J => dfa_state = JO,
      Some('q') if dfa_state == INITIAL => dfa_state = Q,
      Some('s') if dfa_state == JOB => dfa_state = JOBS,
      Some('t') if dfa_state == QUI => dfa_state = QUIT,
      Some('u') if dfa_state == Q => dfa_state = QU,
      Some(' ') => (),
      Some(ch) if char::is_digit(ch) => {
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
      Some(_) => return None
    }
  };
  return None;
}
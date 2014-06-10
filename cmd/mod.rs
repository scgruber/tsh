use std::io;
use std::fmt;

mod dfa;

/// Kinds of built-in commands to tsh. These supersede programs of the same 
/// name.
pub enum BuiltinCmd {
  Quit,
  Jobs,
  Background,
  Foreground
}

impl fmt::Show for BuiltinCmd {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Quit => write!(f, "quit"),
      Jobs => write!(f, "jobs"),
      Background => write!(f, "bg"),
      Foreground => write!(f, "fg")
    }
  }
}

/// Kinds of commands entered by the user.
pub enum Cmd {
  Exec(String, Vec<String>), // Executable, Arguments
  Builtin(BuiltinCmd),
  Error,
  Null
}

impl fmt::Show for Cmd {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Exec(ref prog, ref args) => write!(f, "exec {} {}", prog, args),
      Builtin(cmd) => write!(f, "builtin {}", cmd),
      Error => write!(f, "error"),
      Null => write!(f, "()")
    }
  }
}

/// Extracts a Cmd from the user's input.
pub fn parse_cmd(user_input : Result<String,io::IoError>) -> Cmd {
  match user_input {
    Ok(ref input) => {
      match dfa::builtin_parse_dfa(input.as_slice()) {
        Some(builtin) => Builtin(builtin),
        None => if input.as_slice().trim() == "" {
                  Null
                } else {
                  Exec(from_str::<String>(input.as_slice().trim()).unwrap(), vec![])
                }
      }
    },
    Err(ref e) => Error
  }
}

#[test]
fn parse_cmd_test_quit() {
  let input = Ok("quit\n".to_string());
  match parse_cmd(input) {
    Builtin(Quit) => (),
    _ => fail!()
  }
}

#[test]
fn parse_cmd_test_whitespace() {
  let input = Ok("    \n".to_string());
  match parse_cmd(input) {
    Null => (),
    _ => fail!()
  }
}

#[test]
fn parse_cmd_test_foocmd() {
  let input = Ok("foocmd\n".to_string());
  match parse_cmd(input) {
    Exec(cmd, args) => {
      assert_eq!(cmd, "foocmd".to_string());
      assert_eq!(args, vec![]);
    },
    _ => fail!()
  }
}
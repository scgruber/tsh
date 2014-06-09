use std::io;
use std::fmt;

mod dfa;

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
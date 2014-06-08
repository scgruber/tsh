use std::io;
use std::fmt;

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
  Error
}

pub fn parse_cmd(user_input : Result<String,io::IoError>) -> Cmd {
  match user_input {
    Ok(input) => Exec(input, vec![]),
    Err(e) => Error
  }
}
use std::io;

pub enum BuiltinCmd {
  Quit,
  Jobs,
  Background,
  Foreground
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
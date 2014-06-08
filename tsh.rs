use std::io;

enum Cmd {
  Exec,
  Builtin,
  Error
}

fn get_cmd_from_stdin() -> Cmd{
  let command = io::stdin().read_line();
  match command {
    Ok(input) => Exec,
    Err(e) => Error
  }
}

fn main() {
  loop {
    print!("tsh> ");
    let command = get_cmd_from_stdin();
    println!("Received command");
  }
}
use std::io;

enum Cmd {
  Exec(String, Vec<String>), // Executable, Arguments
  Builtin,
  Error
}

fn get_cmd_from_stdin() -> Cmd{
  let command = io::stdin().read_line();
  match command {
    Ok(input) => Exec(input, vec![]),
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
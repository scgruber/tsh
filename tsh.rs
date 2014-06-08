use std::io;

mod cmd;

fn main() {
  loop {
    print!("tsh> ");
    let command = cmd::parse_cmd(io::stdin().read_line());
    println!("Received command");
  }
}
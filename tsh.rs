use std::io;

fn main() {
  loop {
    print!("tsh> ");
    let command = io::stdin().read_line();
    println!("Received command {}", command);
  }
}
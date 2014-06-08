use std::io;

mod cmd;

fn main() {
  loop {
    print!("tsh> ");
    let command = cmd::parse_cmd(io::stdin().read_line());
    match command {
      cmd::Exec(ref prog, ref args) => println!("{}", command),
      cmd::Builtin(ref cmd) => println!("{}", command),
      cmd::Error => {
        println!("There was an error!");
        return
      },
      cmd::Null => ()
    }
  }
}
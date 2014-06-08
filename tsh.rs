use std::io;

mod cmd;

fn main() {
  loop {
    print!("tsh> ");
    let command = cmd::parse_cmd(io::stdin().read_line());
    // Decide everything to do in one big match expression!
    match command {
      cmd::Exec(ref prog, ref args) => println!("{}", command),
      cmd::Builtin(cmd::Quit) => {
        println!("Exiting.");
        return
      },
      cmd::Builtin(cmd::Jobs) => {
        println!("`jobs` Unimplemented")
      },
      cmd::Builtin(cmd::Foreground) => {
        println!("`fg` Unimplemented")
      },
      cmd::Builtin(cmd::Background) => {
        println!("`bg` Unimplemented")
      },
      cmd::Error => {
        println!("There was an error!");
        return
      },
      cmd::Null => ()
    }
  }
}
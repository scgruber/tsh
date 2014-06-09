use std::io;
use std::io::Command;
use std::io::process;

mod cmd;
mod job;

/// Accepts commands from the user.
fn main() {
  // Initialize jobs list
  let mut jobs = job::JobsList::new();
  loop {
    print!("tsh> ");
    let command = cmd::parse_cmd(io::stdin().read_line());
    // Decide everything to do in one big match expression!
    match command {
      cmd::Exec(ref prog, ref args) => exec(prog.as_slice(), args.as_slice()),
      cmd::Builtin(cmd::Quit) => {
        println!("Exiting.");
        return
      },
      cmd::Builtin(cmd::Jobs) => {
        println!("{}", jobs)
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

/// Runs a command in a separate process, blocking tsh until the command 
/// finishes.
fn exec(prog: &str, args: &[String]) {
  match Command::new(prog).args(args)
                          .stdin(process::InheritFd(0))
                          .stdout(process::InheritFd(1))
                          .stderr(process::InheritFd(2))
                          .spawn() {
                      Ok(mut p) => {
                        p.wait();
                      },
                      Err(e) => {
                        println!("Error attempting to execute `{} {}`", prog, args);
                      }
                    };
}
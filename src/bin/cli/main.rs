use std::io::{self, prelude::*};

use sqruste::command::Command;
mod metacmd;
use metacmd::MetaCommand;

fn print_prompt() {
    _ = io::stdout().flush();
    print!("db > ");
    _ = io::stdout().flush();
}

fn main() {
    println!("Hello World!!");
    let mut stdin = io::stdin().lock().lines();
    loop {
        print_prompt();
        let Some(line) = stdin.next() else { break };
        let Ok(line) = line else { continue };
        match line.as_str() {
            s if s.starts_with('.') => match MetaCommand::run(s) {
                Err(metacmd::MetaCommandError::Unrecognized) => {
                    println!("unrecognized command `{s}`")
                }
                _ => {}
            },
            s => match Command::prepare(s) {
                Err(e) => println!("{e:#?}"),
                _ => {}
            },
            // _ => println!("Unrecofnized command {line}."),
        }
    }
}

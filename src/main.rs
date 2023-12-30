use std::{path::Path, fs::File, io::Read};

mod json; 
mod cli;
mod git;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let args = args.join("");
    
    let mut cli = cli::ScorchProjectCLI::new();
    if args.is_empty() || cli.try_command(args).is_none() {
        let result = cli.run_repl(); // run the repl if no command is given
        match result {
            Ok(_) => {},
            Err(_) => {
                //println!("Error: {}", err);
            }
        }
    }
}
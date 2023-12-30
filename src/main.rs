use std::{path::Path, fs::File, io::Read};

mod json; 
mod cli;
mod git;

fn main() {
    
    let args = std::env::args().collect::<Vec<String>>();
    
    if args.len() == 1 {
        let mut cli = cli::ScorchProjectCLI::new();
        cli.run_cli().unwrap();
        return;
    }
    
    let arg1 = args[1].as_str();
    match arg1 {
        "r" => {
            let mut cli = cli::ScorchProjectCLI::new();
            cli.try_load_project_from_dir();
            cli.try_run_current_project();  
        }
        _ => {
            if arg1.ends_with(".scorch") {
                println!("running... {:?}", arg1);
                let path = Path::new(&arg1);
                let mut file = File::open(&path).expect("Unable to open file");
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Unable to read file");
                println!("{:?}", scorch_lang::run(&contents));
                return;
            }
            println!("unknown argument: {}. please try again.", args[1]);
        }
    }
}
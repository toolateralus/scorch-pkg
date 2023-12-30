mod json; 
mod cli;
mod git;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let args = args.join("");
    
    let mut cli = cli::ScorchProjectCLI::new();
    
    if args.is_empty() {
        let _ = cli.run_repl();
        return;
    }
    
    cli.try_command(args);
}
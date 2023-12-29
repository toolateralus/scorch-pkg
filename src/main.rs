mod json; 
mod cli;

fn main() {
    
    let args = std::env::args().collect::<Vec<String>>();
    
    if args.len() > 1 && args[1] == "r" {
        
        let mut cli = cli::ScorchProjectCLI::new();
        cli.try_load_project_from_dir();
        cli.try_run_current_project();  
        
        
        return;
    }
    
    
    let mut cli = cli::ScorchProjectCLI::new();
    
    cli.run_cli().unwrap();    
}
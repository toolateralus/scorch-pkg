use std::{io::Read, collections::HashMap, ops::ControlFlow};

use crate::json::{ScorchProject, FILE_EXTENSION};

pub struct ScorchProjectCLI {
    pub root : String,
    pub project: Option<Box<ScorchProject>>,
}
impl ScorchProjectCLI {
    pub fn run_cli(&mut self) -> Result<(), String> {
        loop {
            let mut input = String::new();
            
            println!("enter a command or type 'help' for a list of commands");
            
            std::io::stdin().read_line(&mut input).unwrap();
            
            let input_vec: Vec<&str> = input.trim().split(" ").collect();
            
            match input_vec[0] {
                "help" => {
                    println!("available commands:");
                    println!("dir : print the current directory.");
                    println!("l <project-path> : load a project from a file path.");
                    println!("r : run the currently loaded project.");
                    println!("create : create a new project at 'dir/project-name.scproj'.'");
                    println!("exit : exit the cli.");
                },
                "l" => {
                    if let ControlFlow::Break(_) = self.load_project(input_vec) {
                        continue;
                    }
                }
                "r" => {
                    self.try_run_current_project();
                }
                "dir" => {
                    println!("current dir : {}\n preview package name : {}/{}", self.root, self.root, "my_project.scproj");
                }
                "create" => {
                    
                    self.create_new_project_cli();
                },
                "exit" => {
                    return Ok(());
                }
                _ => {
                    println!("Unknown command: {}", input);
                    continue;
                }
            }
        }
    }

    fn load_project(&mut self, input_vec: Vec<&str>) -> ControlFlow<()> {
        if input_vec.len() > 1 {
            let file_path = input_vec[1].to_string();
    
            let proj = ScorchProject::load(&file_path);
    
            if !proj.is_ok() {
                println!("Error loading project: {:#?}", proj);
                return ControlFlow::Break(());
            }
    
            let proj = proj.unwrap();
    
            println!("{:?} loaded. use 'r' command to launch from the main specified in the project file", proj.name);
    
            self.project = Some(Box::new(proj));
        } else {
            println!("enter a project file path like 'my_project.scproj' or 'project.scproj'");
        }
        ControlFlow::Continue(())
    }

    fn create_new_project_cli(&mut self) {
        println!("Creating a project at {}", self.root);
                    
        println!("enter a project name like 'my_project' or 'project'");
                    
        let mut input = String::new();
                    
        std::io::stdin().read_line(&mut input).unwrap();
                    
        let name = input.trim().replace("\n", "").clone();
                    
        input.clear();
                    
        println!("enter a main file name like 'main.scorch'");
                    
        std::io::stdin().read_line(&mut input).unwrap();
                    
        let main = input.trim().replace("\n", "").clone();
                    
        let proj = ScorchProject {
            name: name.clone(),
            main,
            module_paths: Vec::new(),
            config: HashMap::new(),
        };
        proj.save(format!("{}/{}{}", self.root, name, FILE_EXTENSION).as_str()).unwrap();
    }

    pub fn try_run_current_project(&mut self) -> () {
        if let Some(proj) = &self.project {
            println!("running project: {}", proj.name);
            println!("main: {}", proj.main);
            println!("modules: {:?}", proj.module_paths);
            println!("config: {:?}", proj.config);
    
            let mut module_files = Vec::new();
    
            for module_path in &proj.module_paths {
                let mut module_file = std::fs::File::open(format!("{}/{}", self.root, module_path)).unwrap();
                let mut module_buffer = String::new();
                module_file.read_to_string(&mut module_buffer).unwrap();
                module_files.push(module_buffer);
            }
    
            let mut main_file = std::fs::File::open(format!("{}/{}", self.root, proj.main)).unwrap();
            let mut main_buffer = String::new();
            main_file.read_to_string(&mut main_buffer).unwrap();
    
            let concatenated_code = format!("{}\n{}", module_files.join("\n"), main_buffer);
            let result = scorch_lang::run(&concatenated_code);
    
            let Ok(return_value) = result else {
                println!("Error running project: {:#?}", result);
                return;
            };
    
            dbg!(return_value);
    
        } 
        else {
            self.try_load_project_from_dir();
            println!("No project loaded.\nsearching current dir & running first found .scproj\noptionally, use 'l' command to load a project");
        }
    }
    
    pub fn try_load_project_from_dir(&mut self) {
        // Search the root directory for .scproj files
        let entries = std::fs::read_dir(&self.root).unwrap();
                        
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
    
            if path.is_file() && path.extension().and_then(std::ffi::OsStr::to_str) == Some("scproj") {
                let proj = ScorchProject::load(path.to_str().unwrap());
        
                if !proj.is_ok() {
                    println!("Error loading project: {:#?}", proj);
                    continue;
                }
        
                let proj = proj.unwrap();
        
                println!("{:?} loaded. use 'r' command to launch from the main specified in the project file", proj.name);
        
                self.project = Some(Box::new(proj));
                break;
            }
        }
    }
    
    pub fn new() -> Self {
        return Self {
            root: std::env::current_dir().unwrap().to_str().unwrap().to_string(),
            project: None,
        }
    }
}
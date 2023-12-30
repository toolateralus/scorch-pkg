use std::{io::Read, collections::HashMap, ops::ControlFlow};

use crate::{json::{ScorchProject, FILE_EXTENSION}, git::cache_repo};
use colored::Colorize;
use std::fs;
use std::path::Path;
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
                    println!("{}", "available commands:");
                    println!("{}", "## <man>               :        A list of built-in functions                             ##".bright_green());
                    println!("{}", "## <dir>               :        print the current directory.                             ##".bright_green());
                    println!("{}", "## <l> 'path'          :        load a project from a file path.                         ##".bright_green());
                    println!("{}", "## <r>                 :        run the currently loaded project.                        ##".bright_green());
                    println!("{}", "## <create>            :        create a new project at 'dir/project-name.scproj'.'      ##".bright_green());
                    println!("{}", "## <exit>              :        exit the cli.                                            ##".bright_green());
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
                    
                    let string = format!("current dir : {}\n preview package name : {}/{}", self.root, self.root, "my_project.scproj");
                    println!("{}", string.green());
                }
                "create" => {
                    
                    self.create_new_project_cli();
                },
                "exit" => {
                    return Ok(());
                }
                "clear" => {
                    
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
            includes: Vec::new(),
            modules: Vec::new(),
        };
        
        proj.save(format!("{}/{}{}", self.root, name, FILE_EXTENSION).as_str()).unwrap();
        
        println!("project created at {}/{}{}", self.root, name, FILE_EXTENSION);
    }
    
    fn load_scorch_files(repo_path: &str) -> Vec<String> {
        let mut scorch_files = Vec::new();
        
        if let Ok(entries) = fs::read_dir(repo_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() && path.extension().map(|ext| ext == "scorch").unwrap_or(false) {
                        if let Ok(file_content) = fs::read_to_string(&path) {
                            scorch_files.push(file_content);
                        }
                    } else if path.is_dir() {
                        let files = Self::load_scorch_files(path.to_str().unwrap());
                        scorch_files.extend(files);
                    }
                }
            }
        }

        scorch_files
    }
    
    pub fn try_run_current_project(&mut self) -> () {
        if let Some(proj) = &self.project {
            println!("running project: {}", proj.name);
            println!("main: {}", proj.main);
            println!("included files: {:?}", proj.includes);
            println!("git modules: {:?}", proj.modules);
            
            let mut module_cache = HashMap::new();
            
            for module in &proj.modules {
                let id = module.id.clone();
                let url = module.url.clone();
                let branch = module.branch.clone();
                
                let result = cache_repo(&id, &url, &branch);
                
                let Ok(repo_path) = result else {
                    println!("Error caching repo: {:#?}", result);
                    return;
                };
                
                let scorch_files = Self::load_scorch_files(&repo_path);
                
                module_cache.insert(id, scorch_files);
            }
            
            let mut module_files = Vec::new();
            for module_path in &proj.includes {
                match std::fs::File::open(format!("{}/{}", self.root, module_path)) {
                    Ok(mut module_file) => {
                        let mut module_buffer = String::new();
                        match module_file.read_to_string(&mut module_buffer) {
                            Ok(_) => module_files.push(module_buffer),
                            Err(e) => {
                                println!("Error reading module file: {:#?}", e);
                                return;
                            }
                        }
                    },
                    Err(e) => {
                        println!("Error opening module file: {:#?}", e);
                        return;
                    }
                }
            }
    
            let Ok(mut main_file) = std::fs::File::open(format!("{}/{}", self.root, proj.main)) else {
                println!("Error opening main file: {:#?}", proj.main);
                return;
            };
            
            
            let mut main_buffer = String::new();
            let Ok(_) = main_file.read_to_string(&mut main_buffer) else {
                println!("Error reading main file: {:#?}", proj.main);
                return;
            };
    
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
            self.try_run_current_project();
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
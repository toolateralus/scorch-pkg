use std::{io::Read, ops::ControlFlow, fs::File};

use crate::{json::{ScorchProject, FILE_EXTENSION}, git::{try_cache_repo, force_update_repo}};
use colored::Colorize;
use indexmap::IndexMap;
use std::fs;
use std::path::Path;
pub struct ScorchProjectCLI {
    pub root : String,
    pub project: Option<Box<ScorchProject>>,
}
impl ScorchProjectCLI {
    pub fn run_repl(&mut self) -> Result<(), String> {
        loop {
            let mut input = String::new();
            
            println!("enter a command or type 'help' for a list of commands");
            
            std::io::stdin().read_line(&mut input).unwrap();
            
            if let Some(value) = self.try_command(input) {
                return value;
            }
        }
    }

    pub fn try_command(&mut self, input: String) -> Option<Result<(), String>> {
        let input_vec: Vec<&str> = input.trim().split(" ").collect();
        let arg1 = input_vec[0];
        
        match arg1 {
            "update" => {
                self.try_load_project_from_dir();
                if self.project.is_none() {
                    println!("you must have a project loaded or be in the directory of a project to update & pull from module remotes");
                    return Some(Ok(()));
                }
        
                for repos in &self.project.as_ref().unwrap().modules {
                    
                    
                    let id = repos.id.clone();
                    let url = repos.url.clone();
                    let branch = repos.branch.clone();
            
                    println!("updating git dependency: \nmodule: {}\nfrom repo: {}\non branch {}", id.clone(), url, branch);
                    
                    let result = force_update_repo(&id, &url, &branch);
            
                    let Ok(repo_path) = result else {
                        println!("Error caching repo: {:#?}", result);
                        return Some(Ok(()));
                    };
                    println!("successfully updated repo {} from {} , locally at {}", id.clone(), url, repo_path)                        
                }
            }
            "help" => {
                println!("{}", "available commands:");
                println!("{}", "## <dir>               :        print the current directory.                             ##".bright_green());
                println!("{}", "## <l> 'path'          :        load a project from a file path.                         ##".bright_green());
                println!("{}", "## <r>                 :        run the currently loaded project.                        ##".bright_green());
                println!("{}", "## <create>            :        create a new project at 'dir/project-name.scproj'.'      ##".bright_green());
                println!("{}", "## <exit>              :        exit the cli.                                            ##".bright_green());
            },
            "l" => {
                self.load_project(input_vec);
            }
            "r" => {
                self.try_load_project_from_dir();
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
                return Some(Ok(()));
            }
            "clear" => {
                //scorch_lang::clear_screen();
            }
            _ => {
                if arg1.ends_with(".scorch") {
                    println!("running... {:?}", arg1);
                    let path = Path::new(&arg1);
                    let mut file = File::open(&path).expect("Unable to open file");
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).expect("Unable to read file");
                    println!("{:?}", scorch_lang::run(&contents));
                }
                println!("unknown argument: {}. please try again.", arg1);
            }
        }
        None
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
            
            let mut module_cache = IndexMap::new();
            
            for module in &proj.modules {
                let id = module.id.clone();
                let url = module.url.clone();
                let branch = module.branch.clone();
                
                let result = try_cache_repo(&id, &url, &branch);
                
                let Ok(repo_path) = result else {
                    println!("Error caching repo: {:#?}", result);
                    return;
                };
                
                let scorch_files = Self::load_scorch_files(&repo_path);
                
                module_cache.insert(id, scorch_files);
            }
            
            for include in &proj.includes {
                let path = format!("{}/{}", self.root, include);
                let path = std::path::Path::new(&path);
                if path.is_dir() {
                    for entry in std::fs::read_dir(&path).unwrap() {
                        let entry = entry.unwrap();
                        let content = Self::read_file_content(&entry.path());
                        module_cache.insert(entry.file_name().to_str().unwrap().to_string(), vec![content]);
                    }
                } else {
                    let content = Self::read_file_content(&path);
                    module_cache.insert(include.clone(), vec![content]);
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
    
            module_cache.insert("main".to_string(), vec![main_buffer]);
    
            let result = scorch_lang::run_with_modules(module_cache);
    
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
    fn read_file_content(path: &Path) -> String {
        let mut file = File::open(path).expect("Unable to open file");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Unable to read file");
        content
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
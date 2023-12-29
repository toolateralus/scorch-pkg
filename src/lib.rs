use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

use serde::{Deserialize, Serialize};

pub const FILE_EXTENSION: &str = ".scproj";

#[derive(Debug, Serialize, Deserialize)]
pub struct ScorchProject {
    pub name: String,
    pub main: String,
    pub module_paths: Vec<String>,
    pub config: HashMap<String, String>,
}

impl ScorchProject {
    pub fn save(&self, file_path: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;
    
        let serialized = serde_json::to_string_pretty(self)?;
    
        file.write_all(serialized.as_bytes())?;
    
        Ok(())
    }
    
    pub fn load(file_path: &str) -> io::Result<Self> {
        let mut file = File::open(file_path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        let deserialized: Self = serde_json::from_str(&buffer)?;

        Ok(deserialized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_save_and_load() {
        let project = ScorchProject {
            name: "ScorchTestProject".to_string(),
            main: "main.scorch".to_string(),
            module_paths: vec!["module1.scorch".to_string(), "module2.scorch".to_string()],
            config: HashMap::from([("libstd".to_string(), "true".to_string())]),
        };
        
        project.save("test_project.json").unwrap();
        
        let loaded_project = ScorchProject::load("test_project.json").unwrap();
        
        assert_eq!(project.name, loaded_project.name);
        assert_eq!(project.main, loaded_project.main);
        assert_eq!(project.module_paths, loaded_project.module_paths);
        assert_eq!(project.config, loaded_project.config);
    }
}


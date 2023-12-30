use git2::build::CheckoutBuilder;
use std::env;
use git2::{Repository, Error, ObjectType};
use std::path::{Path, PathBuf};
use std::fs;

#[cfg(target_os = "windows")]
pub const GIT_CACHE: &str = "appdata/scorch/git_cache";

#[cfg(target_os = "linux")]
pub const GIT_CACHE: &str = ".config/scorch/git_cache";

pub fn get_git_cache_path() -> PathBuf {
    let home_dir = env::var("HOME").expect("HOME environment variable is not set.");
    let mut path = PathBuf::from(home_dir);
    path.push(GIT_CACHE);
    path
}


pub fn cache_repo(id: &str, url: &str, branch: &str, cache_dir: &Path) -> Result<(), Error> {
    let repo_dir = format!("{}/{}", cache_dir.display(), id);
    
    // If the directory exists, delete it
    if Path::new(&repo_dir).exists() {
        fs::remove_dir_all(&repo_dir)
            .map_err(|err| Error::from_str(&format!("Failed to remove existing directory: {}", err)))?;
    }
    
    let repo = Repository::clone(url, &repo_dir)
        .map_err(|err| Error::from_str(&format!("Failed to clone repository: {}", err)))?;
    
    let obj = repo.revparse_single(branch)
        .map_err(|err| Error::from_str(&format!("Failed to parse branch: {}", err)))?;
    
    match obj.kind() {
        Some(ObjectType::Commit) | Some(ObjectType::Tag) => {
            let mut checkout_builder = CheckoutBuilder::new();
            checkout_builder.force();
            repo.checkout_tree(&obj, Some(&mut checkout_builder))
                .map_err(|err| Error::from_str(&format!("Failed to checkout tree: {}", err)))?;
            repo.set_head_detached(obj.id())
                .map_err(|err| Error::from_str(&format!("Failed to detach head: {}", err)))?;
        },
        _ => return Err(Error::from_str("Invalid object type")),
    }
    
    Ok(())
}

mod test {
    use super::*;
    #[test]
    fn test_cache_repo() {
        let cache_dir = get_git_cache_path();
        
        let id = "scorch-doc";
        let url = "https://github.com/toolateralus/scorch-doc.git";
        let branch = "main";
        
        let result = cache_repo(id, url, branch, &cache_dir);
        
        dbg!(&result);
        
        assert!(result.is_ok(), "git repo cache test failed,");
    }
}
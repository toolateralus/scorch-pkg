use git2::build::CheckoutBuilder;
use std::{env, path::Path};
use git2::{Repository, Error, ObjectType};
use std::path::PathBuf;

#[cfg(target_os = "windows")]
pub const GIT_CACHE: &str = "appdata/roaming/scorch/git_cache";

#[cfg(target_os = "linux")]
pub const GIT_CACHE: &str = ".config/scorch/git_cache";

pub fn get_git_cache_path() -> PathBuf {
    let home_dir = env::var("HOME").expect("HOME environment variable is not set.");
    let mut path = PathBuf::from(home_dir);
    path.push(GIT_CACHE);
    path
}

pub fn try_cache_repo(id: &str, url: &str, branch: &str) -> Result<String, Error> {
    let repo_dir = get_repo_directory(id);
    if Path::new(&repo_dir).exists() {
        return Ok(repo_dir);
    }
    force_update_repo(&id.to_string(), url, branch)
}

pub fn force_update_repo(id: &String, url: &str, branch: &str) -> Result<String, Error> {
    let repo_dir = get_repo_directory(id);
    let repo = open_or_clone_repo(&repo_dir, url)?;
    update_repo_if_needed(&repo, branch)?;
    checkout_branch(&repo, branch)?;
    
    Ok(repo_dir.clone())
}

pub fn get_repo_directory(id: &str) -> String {
    let cache_dir = get_git_cache_path();
    format!("{}/{}", cache_dir.display(), id)
}

pub fn open_or_clone_repo(repo_dir: &str, url: &str) -> Result<Repository, Error> {
    match Repository::open(repo_dir) {
        Ok(repo) => Ok(repo),
        Err(_) => Repository::clone(url, repo_dir)
            .map_err(|err| Error::from_str(&format!("Failed to clone repository: {}", err))),
    }
}

pub fn update_repo_if_needed(repo: &Repository, branch: &str) -> Result<(), Error> {
    repo.find_remote("origin")?
        .fetch(&[branch], None, None)?;
    
    let local_commit = repo.revparse_single("HEAD")?.id();
    let remote_commit = repo.revparse_single(&format!("origin/{}", branch))?.id();
    if local_commit != remote_commit {
        let object = repo.revparse_single(&format!("origin/{}", branch))?;
        repo.reset(&object, git2::ResetType::Hard, None)?;
    }

    Ok(())
}

pub fn checkout_branch(repo: &Repository, branch: &str) -> Result<(), Error> {
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
       
    #[test]
    pub fn test_cache_repo() {
        let id = "scorch-doc";
        let url = "https://github.com/toolateralus/scorch-doc.git";
        let branch = "main";
        
        let result = super::try_cache_repo(id, url, branch);
        
        match &result {
            Ok(path) => {
                let path = std::path::Path::new(path);
                assert!(path.exists(), "git repo cache test failed, path does not exist");
                assert!(path.is_dir(), "git repo cache test failed, path is not a directory");
                std::fs::remove_dir_all(path).expect("git repo cache test failed, failed to remove directory");
            },
            Err(_) => {},
        }
    }
}
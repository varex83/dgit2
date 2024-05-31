use anyhow::anyhow;
use anyhow::Result;
use std::env;

pub fn detect_git_dir() -> Result<String> {
    let mut current_dir = env::current_dir()?;
    loop {
        let git_dir = current_dir.join(".git");
        if git_dir.exists() {
            let git_dir = git_dir.to_str().unwrap().to_string();

            return Ok(git_dir);
        }
        if !current_dir.pop() {
            break;
        }
    }
    Err(anyhow!("Not a git repository"))
}

use anyhow::anyhow;
use anyhow::Result;
use std::env;

pub fn detect_git_dir() -> Result<String> {
    let mut current_dir = env::current_dir()?;
    loop {
        let read_dir = std::fs::read_dir(current_dir.as_path())?;

        for entry in read_dir {
            let entry = entry?;
            let path = entry.path();
            if path.ends_with(".git") {
                return Ok(path.to_str().unwrap().to_string());
            }
        }

        if !current_dir.pop() {
            break;
        }
    }
    Err(anyhow!("Not a git repository"))
}

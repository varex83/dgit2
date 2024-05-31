use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;

pub async fn get_object_hashes(git_objects_path: &str) -> Result<(Vec<String>, Vec<String>)> {
    let mut object_hashes = Vec::new();
    let mut object_paths = Vec::new();

    if Path::new(git_objects_path).exists() {
        for entry in WalkDir::new(git_objects_path)
            .into_iter()
            .filter_map(Result::ok)
        {
            let path = entry.path();

            if path.is_file() {
                if let Some(relative_path) = path
                    .strip_prefix(git_objects_path)
                    .ok()
                    .and_then(|p| p.to_str())
                {
                    let hash = relative_path.replace('/', "");
                    object_hashes.push(hash);
                    object_paths.push(path.to_str().unwrap().to_string());
                }
            }
        }
    }

    Ok((object_hashes, object_paths))
}

pub fn get_refs(git_refs_path: &str) -> Result<Vec<String>> {
    let mut refs = Vec::new();

    if Path::new(git_refs_path).exists() {
        for entry in WalkDir::new(git_refs_path)
            .into_iter()
            .filter_map(Result::ok)
        {
            let path = entry.path();

            if path.is_file() {
                if let Some(relative_path) = path
                    .strip_prefix(git_refs_path)
                    .ok()
                    .and_then(|p| p.to_str())
                {
                    refs.push(relative_path.to_string());
                }
            }
        }
    }

    Ok(refs)
}

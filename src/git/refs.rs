use crate::git::utils::detect_git_dir;
use anyhow::Result;
use async_recursion::async_recursion;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Ref(pub String);

impl Ref {
    // get ref path
    pub fn new(ref_name: &str) -> Self {
        Ref(ref_name.trim().to_string())
    }

    // get ref content
    pub async fn get_content(&self) -> Result<Vec<u8>> {
        let path = format!("./.git/{}", self.0);

        let content = tokio::fs::read(path).await.unwrap_or(vec![]);

        Ok(content)
    }

    #[async_recursion]
    pub async fn resolve(&self) -> Result<String> {
        let content = self.get_content().await?;

        let content = String::from_utf8(content)?;

        let content = Ref::parse_ref_content(&content);

        if content.starts_with("refs/") {
            let ref_ = Ref::new(&content);
            ref_.resolve().await
        } else {
            Ok(content)
        }
    }

    pub async fn update(&mut self, hash: &str) -> Result<()> {
        let path = format!("./.git/{}", self.0);

        // create parent directories if they don't exist
        let parent = std::path::Path::new(&path).parent().unwrap();
        let _ = tokio::fs::create_dir_all(parent).await;

        tokio::fs::write(path, hash).await?;

        Ok(())
    }

    pub fn parse_ref_content(content: &str) -> String {
        if content.starts_with("ref:") {
            let ref_name = content.split(':').collect::<Vec<&str>>()[1].trim();
            ref_name.to_string()
        } else {
            let content = content.trim();
            content.to_string()
        }
    }

    pub async fn get_all_refs() -> Result<Vec<(String, Vec<u8>)>> {
        // find .git directory
        let git_dir = detect_git_dir()?;

        let refs = format!("{}/refs", git_dir);

        let mut result = Vec::new();

        for entry in WalkDir::new(Path::new(&refs))
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            if path.is_dir() {
                continue;
            }

            let path = path.to_str().unwrap();
            let path = path.replace("\\", "/");

            let path = path.split('/').collect::<Vec<&str>>();

            let ref_path = path.join("/");

            // get relative path
            let ref_path = ref_path.split(".git/").collect::<Vec<&str>>()[1];
            let content = tokio::fs::read(entry.path()).await?;

            result.push((ref_path.to_string(), content));
        }

        Ok(result)
    }
}

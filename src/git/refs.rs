use anyhow::Result;
use async_recursion::async_recursion;

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

        tokio::fs::write(path, hash).await?;

        Ok(())
    }

    pub fn parse_ref_content(content: &str) -> String {
        if content.starts_with("ref:") {
            let ref_name = content.split(':').collect::<Vec<&str>>()[1].trim();
            ref_name.to_string()
        } else {
            content.to_string()
        }
    }
}

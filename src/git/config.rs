#[derive(Debug)]
pub struct Config {
    pub repository_address: String,
}

impl Config {
    pub fn new(repository_address: String) -> Self {
        Config { repository_address }
    }

    pub fn get() -> Self {
        // load config from file .git/DGITCONFIG
        let content = std::fs::read("./.git/DGITCONFIG").unwrap();

        let content = String::from_utf8(content).unwrap();

        let repository_address = content.trim();

        Config::new(repository_address.to_string())
    }

    pub async fn save(&self) -> anyhow::Result<()> {
        tokio::fs::write("./.git/DGITCONFIG", self.repository_address.as_bytes()).await?;

        Ok(())
    }
}

use crate::commands::commit;
use crate::git::config::Config;
use anyhow::Result;
use std::fs;

pub async fn init(contract_address: String) -> Result<()> {
    let _ = fs::create_dir(".git");
    let _ = fs::create_dir(".git/objects");
    let _ = fs::create_dir(".git/refs");
    let _ = fs::write(".git/HEAD", "ref: refs/heads/main\n");
    let _ = fs::create_dir(".git/refs/heads");
    let _ = fs::create_dir(".git/refs/tags");
    let _ = fs::create_dir(".git/refs/remotes");

    let commit = commit(None).await?;

    let _ = fs::write(".git/refs/heads/main", commit);

    // verify correct contract address using regex
    let re = regex::Regex::new(r"0x[a-fA-F0-9]{40}")?;

    if !re.is_match(&contract_address) || contract_address.len() != 42 {
        anyhow::bail!("Invalid contract address");
    }

    let config = Config::new(contract_address);

    config.save().await?;

    println!("Initialized git directory");

    Ok(())
}

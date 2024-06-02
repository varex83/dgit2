use crate::commands::{init, pull};
use crate::git::head::update_current_files_to_current_head;

pub async fn clone(contract_address: String) -> anyhow::Result<()> {
    println!(
        "Cloning repository contract with address: {}",
        contract_address
    );

    init(contract_address.clone()).await?;

    pull().await?;

    update_current_files_to_current_head().await?;

    Ok(())
}

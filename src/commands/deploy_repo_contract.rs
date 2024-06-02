use crate::commands::init;
use crate::contract_interaction::ContractInteraction;

pub async fn deploy_repo_contract() -> anyhow::Result<()> {
    println!("Deploying repository contract");

    let contract = ContractInteraction::deploy().await?;
    let address = contract.address();

    println!("Deployed repository contract with address: {}", address);

    init(address).await?;

    Ok(())
}

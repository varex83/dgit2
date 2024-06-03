use crate::commands::init;
use crate::contract_interaction::ContractInteraction;
use colored::Colorize;

pub async fn deploy_repo_contract() -> anyhow::Result<String> {
    println!(
        "{}",
        format!("{}", "Deploying repository contract...").bright_blue()
    );

    let contract = ContractInteraction::deploy().await?;
    let address = contract.address();

    println!(
        "{}",
        format!("Deployed repository contract with address: {}", address).green()
    );

    init(address.clone()).await?;

    Ok(address)
}

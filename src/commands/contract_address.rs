use crate::config::Config;
use colored::Colorize;

pub async fn contract_address() -> anyhow::Result<()> {
    let contract_address = Config::get_contract_address();

    println!(
        "Contract address: {}",
        format!("{}", contract_address).bright_blue()
    );

    Ok(())
}

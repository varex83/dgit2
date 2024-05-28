use clap::Parser;
use dgit2::cli::Cli;
use dgit2::cli::Commands;
use dgit2::commands;
use dgit2::ipfs::load_to_ipfs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let parser = Cli::parse();

    dotenv::dotenv().ok();

    match parser.command {
        Commands::Init {
            directory,
            contract_address,
        } => commands::init(directory, contract_address).await,
        Commands::Sync {
            contract_address,
            full,
        } => commands::sync(contract_address).await,
        Commands::Status { contract_address } => commands::status(contract_address).await,
        Commands::DeployRepositoryContract { contract_address } => {
            commands::deploy_repo_contract(contract_address).await
        }
        Commands::LoadFile { file_path } => {
            let result = load_to_ipfs(file_path.as_ref().unwrap()).await?;

            println!("IPFS hash: {}", result);

            Ok(())
        }
    }
}

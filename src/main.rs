use anyhow::bail;
use clap::Parser;
use dgit2::cli::Cli;
use dgit2::cli::Commands;
use dgit2::commands;
use dgit2::commands::{cat_file, write_tree};
use dgit2::git::head::update_current_files_to_current_head;
use dgit2::ipfs::{download_from_ipfs, load_to_ipfs};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let parser = Cli::parse();

    dotenv::dotenv().ok();

    match parser.command {
        Commands::Init {
            contract_address,
            deploy,
        } => {
            if deploy {
                commands::deploy_repo_contract().await?;
            }

            commands::init(contract_address).await
        }
        Commands::Clone { contract_address } => commands::clone(contract_address).await,
        Commands::Pull => commands::pull().await,
        Commands::Push => commands::push().await,
        Commands::Status => commands::status().await,
        Commands::Deploy => commands::deploy_repo_contract().await,
        Commands::LoadFile { file_path } => {
            load_to_ipfs(file_path.as_ref().unwrap()).await.map(|_| ())
        }
        Commands::DownloadFile { chash, path } => download_from_ipfs(&chash, &path).await,
        Commands::CatFile { pretty_print } => cat_file(pretty_print.as_str()),
        Commands::HashObject { write } => commands::hash_object(write.as_str()),
        Commands::LsTree { path, name_only } => commands::ls_tree(path.as_str(), name_only),
        Commands::WriteTree => write_tree("."),
        Commands::ContractAddress => commands::contract_address().await,
        Commands::Commit { message } => commands::commit(message).await.map(|_| ()),
        Commands::Debug => {
            println!("Debugging");
            update_current_files_to_current_head().await
        }
        #[allow(unreachable_patterns)]
        _ => bail!("Not implemented yet"),
    }
}

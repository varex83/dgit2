use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "dgit")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Initialize a repository")]
    Init {
        /// The contract address of the repository.
        #[arg(short, long)]
        contract_address: String,
        /// Deploy the repository contract.
        #[arg(short, long)]
        deploy: bool,
    },

    #[command(about = "Clone a repository")]
    Clone {
        /// The URL of the repository to clone.
        contract_address: String,
    },

    #[command(about = "Sync the repository")]
    Pull,

    #[command(about = "Push the repository")]
    Push,

    #[command(about = "Get the status of the repository")]
    Status,

    #[command(about = "Deploy a repository contract")]
    Deploy,

    #[command(about = "Get contract address")]
    ContractAddress,

    #[command(about = "Load a file to IPFS")]
    LoadFile {
        /// The path to the file to load.
        #[arg(short, long)]
        file_path: Option<String>,
    },

    #[command(about = "Download a file from IPFS")]
    DownloadFile {
        /// The IPFS hash of the file to download.
        #[arg(short, long)]
        chash: String,
        /// The path to save the downloaded file.
        #[arg(short, long)]
        path: String,
    },

    #[command(about = "Provide contents or details of repository objects")]
    CatFile {
        /// Pretty-print the contents of <object> based on its type.
        #[arg(short, long)]
        pretty_print: String,
    },

    #[command(about = "Create a tree object from the current index")]
    WriteTree,

    #[command(about = "Create a commit object from the current index")]
    Commit {
        /// The commit message.
        #[arg(short, long)]
        message: Option<String>,
    },

    #[command(about = "Compute object ID and optionally create an object from a file")]
    HashObject {
        /// Actually write the object into the object database.
        #[arg(short, long)]
        write: String,
    },

    #[command(about = "List the contents of a tree object")]
    LsTree {
        /// The object to list the contents of.
        path: String,
        /// List only filenames (instead of the "long" output), one per line. Cannot be combined with --object-only.
        #[arg(short, long)]
        name_only: bool,
    },

    Debug,
}

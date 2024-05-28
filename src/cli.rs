use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "dgit")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Init {
        #[arg(short, long)]
        directory: Option<String>,
        #[arg(short, long)]
        contract_address: Option<String>,
    },

    Sync {
        #[arg(short, long)]
        contract_address: Option<String>,
        #[arg(short, long)]
        full: bool,
    },

    Status {
        #[arg(short, long)]
        contract_address: Option<String>,
    },

    DeployRepositoryContract {
        #[arg(short, long)]
        contract_address: Option<String>,
    },

    LoadFile {
        #[arg(short, long)]
        file_path: Option<String>,
    },
}

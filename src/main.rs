use std::path::PathBuf;

use clap::Parser as _;

use ipfs_eth::{eth, ipfs};

#[derive(clap::Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

/// IPFS/Ethereum integration utility.
#[derive(clap::Subcommand, Debug, Clone)]
enum Command {
    /// Uploads the provided file to the IPFS network.
    ///
    /// Uses the API URL specified in $HOME/.ipfs/api, or localhost:5001.
    UploadFile {
        /// Path to the file to upload.
        path: PathBuf,
    },

    /// Deploys the provided Ethereum smart-contract using CID to construct it.
    ///
    /// Requires `solc` binary available in $SOLC_PATH or $PATH.
    DeployContract {
        /// Path to the source .sol file.
        ///
        /// The file should contain exactly one contact with a single `string` argument constructor.
        src: PathBuf,

        /// URL of the Ethereum JSON-RPC API to use.
        #[arg(short, long)]
        url: String,

        /// CID to pass into the smart-contract constructor.
        #[arg(short, long)]
        cid: String,

        /// Path to the keystore of the Ethereum wallet to use.
        #[arg(short, long)]
        keystore_path: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Args::parse();

    match args.command {
        Command::UploadFile { path } => ipfs::upload_file(path)
            .await
            .map(|cid| println!("Successfully uploaded! CID: {cid}"))
            .map_err(|e| e.to_string()),
        Command::DeployContract { src, url, keystore_path, cid } => eth::DeployContract {
            src,
            url,
            constructor_args: cid,
            keystore_path,
            keystore_password: dialoguer::Password::new()
                .with_prompt("Password")
                .interact()
                .unwrap()
                .into(),
        }
        .exec()
        .await
        .map(|c| println!("Successfully deployed! Address: {:?}", c.address()))
        .map_err(|e| e.to_string()),
    }
}

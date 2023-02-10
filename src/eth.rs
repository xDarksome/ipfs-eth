//! Ethereum related capabilities of this library.

use std::path::PathBuf;

use ethers::{
    abi::Tokenize,
    contract::{Contract, ContractFactory},
    prelude::SignerMiddleware,
    providers::{Middleware, Provider, ProviderError},
    signers::{Signer, Wallet, WalletError},
    solc::{error::SolcError, Artifact as _, Solc},
};
use secrecy::{ExposeSecret, SecretString};
use thiserror::Error;

/// Command for deploying Ethereum smart-contracts.
pub struct DeployContract<Args> {
    /// Path to the source .sol file where the contract is defined.
    pub src: PathBuf,

    /// Ethereum JSON API URL.
    pub url: String,

    /// Arguments of the contract's constructor.
    pub constructor_args: Args,

    /// Path to the keystore file of the Ethereum wallet.
    pub keystore_path: PathBuf,

    /// Password to the keystore file.
    pub keystore_password: SecretString,
}

impl<Args> DeployContract<Args> {
    /// Deploys the smart-contract.
    ///
    /// # Errors
    ///
    /// See [`DeployContractError`] for details.
    pub async fn exec(self) -> Result<Contract<impl Middleware>, DeployContractError>
    where
        Args: Tokenize,
    {
        use DeployContractError as E;

        let provider = Provider::try_from(self.url)?;

        let task = || Solc::default().compile_source(self.src);
        let compiled = tokio::task::spawn_blocking(task).await??;

        let mut contracts = compiled.contracts_into_iter();
        let (_, contract) = contracts.next().ok_or(E::NoContracts)?;
        if contracts.next().is_some() {
            return Err(E::TooManyContracts);
        }
        let (abi, bytecode, _) = contract.try_into_parts()?;

        let chain_id = provider.get_chainid().await.map_err(E::GetChainId)?.as_u64();
        let pwd = self.keystore_password.expose_secret();
        let wallet = Wallet::decrypt_keystore(self.keystore_path, pwd)?.with_chain_id(chain_id);
        let client = SignerMiddleware::new(provider, wallet).into();

        ContractFactory::new(abi, bytecode, client)
            .deploy(self.constructor_args)
            .map_err(|e| E::DeploymentError(Box::new(e)))?
            .send()
            .await
            .map_err(|e| E::DeploymentError(Box::new(e)))
    }
}

#[derive(Error, Debug)]
pub enum DeployContractError {
    #[error("Invalid JSON API URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Failed to join blocking tokio task: {0}")]
    TokioTaskError(#[from] tokio::task::JoinError),

    #[error("Failed to compile the source file: {0}")]
    SolcError(#[from] SolcError),

    #[error("Source file doesn't contain any contracts")]
    NoContracts,

    #[error("Source file contains more than one contract")]
    TooManyContracts,

    #[error("Failed to decrypt the keystore: {0}")]
    DecryptKeystore(#[from] WalletError),

    #[error("Failed to get chain ID from JSON RPC API: {0}")]
    GetChainId(ProviderError),

    #[error("Contract deployment failed: {0}")]
    DeploymentError(#[source] Box<dyn std::error::Error>),
}

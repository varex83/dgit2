use crate::config::Config;
use anyhow::Result;
use ethcontract::prelude::*;
use std::str::FromStr;

ethcontract::contract!("artifacts/contracts/RepositoryContract.sol/RepositoryContract.json");

pub struct ContractInteraction {
    pub contract: RepositoryContract,
    pub client: Web3<Http>,
}

pub struct Object {
    pub hash: String,
    pub ipfs_url: Vec<u8>,
    pub pusher: Address,
}

impl ContractInteraction {
    pub fn new() -> Self {
        let http =
            Http::new(&dotenv::var("RPC_URL").unwrap_or("http://localhost:8545".to_string()))
                .unwrap();
        let client = Web3::new(http);

        let account_contract = Config::get_contract_address();

        let contract = RepositoryContract::at(
            &client,
            Address::from_str(account_contract.as_str()).unwrap(),
        );

        ContractInteraction { contract, client }
    }

    pub fn new_with_address(address: &str) -> Self {
        let http =
            Http::new(&dotenv::var("RPC_URL").unwrap_or("http://localhost:8545".to_string()))
                .unwrap();
        let client = Web3::new(http);

        let contract = RepositoryContract::at(&client, Address::from_str(address).unwrap());

        ContractInteraction { contract, client }
    }

    pub async fn save_object(&self, hash: String, ipfs_url: Vec<u8>) -> Result<()> {
        self.contract
            .save_object(hash, Bytes(ipfs_url))
            .send()
            .await
            .map_err(anyhow::Error::from)
            .map(|_| ())
    }

    pub async fn add_ref(&self, reference: String, data: Vec<u8>) -> Result<()> {
        self.contract
            .add_ref(reference, Bytes(data))
            .send()
            .await
            .map_err(anyhow::Error::from)
            .map(|_| ())
    }

    pub async fn update_config(&self, config: Vec<u8>) -> Result<()> {
        self.contract
            .update_config(Bytes(config))
            .send()
            .await
            .map_err(anyhow::Error::from)
            .map(|_| ())
    }

    pub async fn get_config(&self) -> Result<Vec<u8>> {
        self.contract
            .get_config()
            .call()
            .await
            .map(|Bytes(data)| data.to_vec())
            .map_err(anyhow::Error::from)
    }

    pub async fn get_object_by_id(&self, id: U256) -> Result<Object> {
        let (hash, ipfs_url, pusher) = self
            .contract
            .get_object_by_id(id)
            .call()
            .await
            .map_err(anyhow::Error::from)?;

        Ok(Object {
            hash,
            ipfs_url: ipfs_url.0,
            pusher,
        })
    }

    pub async fn get_object(&self, hash: String) -> Result<Object> {
        let (hash, ipfs_url, pusher) = self
            .contract
            .get_object(hash)
            .call()
            .await
            .map_err(anyhow::Error::from)?;

        Ok(Object {
            hash,
            ipfs_url: ipfs_url.0,
            pusher,
        })
    }

    pub async fn is_object_exist(&self, hash: String) -> Result<bool> {
        self.contract
            .is_object_exist(hash)
            .call()
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn check_objects(&self, hashes: Vec<String>) -> Result<Vec<bool>> {
        self.contract
            .check_objects(hashes)
            .call()
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn add_objects(&self, hashes: Vec<String>, ipfs_urls: Vec<Vec<u8>>) -> Result<()> {
        self.contract
            .add_objects(
                hashes,
                ipfs_urls
                    .iter()
                    .map(|e| Bytes(e.clone()))
                    .collect::<Vec<Bytes<Vec<u8>>>>(),
            )
            .send()
            .await
            .map_err(anyhow::Error::from)
            .map(|_| ())
    }

    pub async fn add_refs(&self, references: Vec<String>, data: Vec<Vec<u8>>) -> Result<()> {
        self.contract
            .add_refs(
                references,
                data.iter()
                    .map(|e| Bytes(e.clone()))
                    .collect::<Vec<Bytes<Vec<u8>>>>(),
            )
            .send()
            .await
            .map_err(anyhow::Error::from)
            .map(|_| ())
    }
}

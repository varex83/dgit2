pub struct Config;

impl Config {
    pub fn get_contract_address() -> String {
        dotenv::var("CONTRACT_ADDRESS").unwrap()
    }

    pub fn get_pinata_secret_api_key() -> String {
        dotenv::var("PINATA_SECRET_API_KEY").unwrap()
    }

    pub fn get_pinata_api_key() -> String {
        dotenv::var("PINATA_API_KEY").unwrap()
    }

    pub fn pk() -> String {
        dotenv::var("PK").unwrap()
    }

    pub fn rpc_url() -> String {
        dotenv::var("RPC_URL").unwrap()
    }
}

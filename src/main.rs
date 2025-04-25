use std::{error::Error, fs};

use base64::prelude::*;
use blake2::{Blake2b, Digest, digest::consts::U32};
use dirs::home_dir;
use serde_derive::Deserialize;

const SUI_CONFIG_DIR: &str = ".sui/sui_config";
const SUI_CLIENT_YAML: &str = "client.yaml";
const SUI_ALIASES: &str = "sui.aliases";

#[derive(Deserialize)]
struct ClientConfig {
    active_env: String,
    active_address: String,
}

#[derive(Deserialize)]
struct ClientAliasesAddress<'a> {
    alias: String,
    public_key_base64: &'a [u8],
}

fn to_sui_address(pk: &[u8]) -> String {
    let d = BASE64_STANDARD.decode(pk).unwrap();
    let mut hasher = Blake2b::<U32>::new();
    hasher.update(d);
    let res = hasher.finalize().to_vec();
    "0x".to_owned() + &hex::encode(&res)
}

fn find_active_alias(
    addresses_alias: Vec<ClientAliasesAddress>,
    active_address: String,
) -> Option<String> {
    let address = addresses_alias
        .iter()
        .find(|address_alias| to_sui_address(address_alias.public_key_base64) == active_address);

    address.map(|address| address.alias.clone())
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(
        home_dir()
            .unwrap()
            .join(SUI_CONFIG_DIR)
            .join(SUI_CLIENT_YAML),
    )?;

    let config: ClientConfig = serde_yml::from_str(&contents)?;

    let contents = fs::read_to_string(home_dir().unwrap().join(SUI_CONFIG_DIR).join(SUI_ALIASES))?;

    let addresses_alias: Vec<ClientAliasesAddress> = serde_json::from_str(&contents)?;

    println!(
        "({}/{})",
        config.active_env,
        find_active_alias(addresses_alias, config.active_address).unwrap()
    );
    Ok(())
}

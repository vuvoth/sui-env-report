use base64::prelude::*;
use blake2::{Blake2b, Digest, digest::consts::U32};
use dirs::home_dir;


const SUI_CONFIG_DIR: &str = ".sui/sui_config";
const SUI_CLIENT_YAML: &str = "client.yaml";
const SUI_ALIASES: &str = "sui.aliases";

fn to_sui_address(pk: &[u8]) -> String {
    let d = BASE64_STANDARD.decode(pk).unwrap();
    let mut hasher = Blake2b::<U32>::new();
    hasher.update(d);
    let res = hasher.finalize().to_vec();
    hex::encode(&res)
}

fn main() {
   
}

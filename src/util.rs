use crate::Result;
use crate::serial::{deserialize, serialize};

use sha2::{Digest, Sha256};
use serde_json::Value;
use log::*;

use std::path::{Path, PathBuf};

pub fn join_config_path(file: &PathBuf) -> Result<PathBuf> {
    let mut path = PathBuf::new();
    let dfi_path = Path::new("darkfi");

    match dirs::config_dir() {
        Some(v) => path.push(v),
        // This should not fail on any modern OS
        None => {}
    }

    path.push(dfi_path);
    path.push(file);

    Ok(path)
}

// here we hash the alphanumeric token ID. if it fails, we change the last 4 bytes and hash it
// again, and keep repeating until it works.
pub fn parse_id(token: &Value) -> Result<jubjub::Fr> {
    let tkn_str = token.as_str().unwrap();
    if bs58::decode(tkn_str).into_vec().is_err() {
        // TODO: make this an error
        debug!(target: "CASHIER", "COULD NOT DECODE STR");
    }
    let mut data = bs58::decode(tkn_str).into_vec().unwrap();
    let token_id = deserialize::<jubjub::Fr>(&data);
    if token_id.is_err() {
        let mut counter = 0;
        loop {
            data.truncate(28);
            let serialized_counter = serialize(&counter);
            data.extend(serialized_counter.iter());
            let mut hasher = Sha256::new();
            hasher.update(&data);
            let hash = hasher.finalize();
            let token_id = deserialize::<jubjub::Fr>(&hash);
            if token_id.is_err() {
                counter += 1;
                continue;
            }
            debug!(target: "CASHIER", "DESERIALIZATION SUCCESSFUL");
            let tkn = token_id.unwrap();
            return Ok(tkn);
        }
    }
    unreachable!();
}



#[cfg(test)]
mod tests {
    use crate::serial::{deserialize, serialize};
    use sha2::{Digest, Sha256};

    #[test]
    fn test_jubjub_parsing() {
        // 1. counter = 0
        // 2. serialized_counter = serialize(counter)
        // 3. asset_id_data = hash(data + serialized_counter)
        // 4. asset_id = deserialize(asset_id_data)
        // 5. test parse
        // 6. loop
        let tkn_str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
        println!("{}", tkn_str);
        if bs58::decode(tkn_str).into_vec().is_err() {
            println!("Could not decode str into vec");
        }
        let mut data = bs58::decode(tkn_str).into_vec().unwrap();
        println!("{:?}", data);
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let hash = hasher.finalize();
        let token_id = deserialize::<jubjub::Fr>(&hash);
        println!("{:?}", token_id);
        let mut counter = 0;
        if token_id.is_err() {
            println!("could not deserialize tkn 58");
            loop {
                println!("TOKEN IS NONE. COMMENCING LOOP");
                counter += 1;
                println!("LOOP NUMBER {}", counter);
                println!("{:?}", data.len());
                data.truncate(28);
                let serialized_counter = serialize(&counter);
                println!("{:?}", serialized_counter);
                data.extend(serialized_counter.iter());
                println!("{:?}", data.len());
                let mut hasher = Sha256::new();
                hasher.update(&data);
                let hash = hasher.finalize();
                let token_id = deserialize::<jubjub::Fr>(&hash);
                println!("{:?}", token_id);
                if token_id.is_err() {
                    continue;
                }
                if counter > 10 {
                    break;
                }
                println!("deserialization successful");
                token_id.unwrap();
                break;
            }
        };
    }
}

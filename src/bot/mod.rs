use serde::{Deserialize, Serialize};

use std::fs;

use web3::contract::Contract;
use web3::types::Address;
use web3::{Transport, Web3};

pub mod platform;
pub mod token;

use self::platform::kyber::Kyber;
use self::platform::uniswap::Uniswap;
use self::platform::DecentralizedExchange;
use self::token::Erc20Token;

#[derive(Serialize, Deserialize)]
struct Web3Descriptor {
    name: String,
    address: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    platforms: Vec<Web3Descriptor>,
    tokens: Vec<Web3Descriptor>,
}

pub struct Bot<D: DecentralizedExchange, T: Transport> {
    platforms: Vec<D>,
    tokens: Vec<Erc20Token<T>>,
}

impl<D: DecentralizedExchange, T: Transport> Bot<D, T> {
    pub fn new(web3: Web3<T>, config_file: &str, abi_folder: &str) -> Self {
        let config_data = fs::read_to_string(config_file).expect("Unable to read config file");
        let config: Config = serde_json::from_str(&config_data).unwrap();

        let platforms: Vec<D> = Vec::new();
        let tokens: Vec<Erc20Token<T>> = Vec::new();

        for platform in config.platforms {
            let separator = if abi_folder.ends_with("/") { "" } else { "/" };
            let abi_filepath = format!("{}{}{}.json", abi_folder, separator, &platform.name);
            let abi_bytes_vector = fs::read(abi_filepath).unwrap();
            let abi_bytes_array: &[u8] = &abi_bytes_vector;

            let platform_addr: Address = platform.address.as_str().parse().unwrap();

            let contract = Contract::from_json(web3.eth(), platform_addr, abi_bytes_array).unwrap();

            let dex_platform: Option<dyn DecentralizedExchange> = match platform.name.as_str() {
                "kyber" => Some(Kyber::new(contract)),
                "uniswap" => Some(Uniswap::new(contract)),
                _ => None,
            };

            if let Some(dex) = dex_platform {
                platforms.append(dex);
            }
        }

        for token in config.tokens {
            let separator = if abi_folder.ends_with("/") { "" } else { "/" };
            let abi_filepath = format!("{}{}erc20.json", abi_folder, separator);
            let abi_bytes_vector = fs::read(abi_filepath).unwrap();
            let abi_bytes_array: &[u8] = &abi_bytes_vector;

            let token_addr: Address = token.address.as_str().parse().unwrap();

            let contract = Contract::from_json(web3.eth(), token_addr, abi_bytes_array).unwrap();

            let erc20 = Erc20Token {
                name: token.name,
                contract,
            };
            tokens.append(erc20);
        }

        Bot { platforms, tokens }
    }
}

pub mod platform;
pub mod token;

use self::platform::{Exchange, Kyber, Platform, Uniswap};
use serde::{Deserialize, Serialize};
use web3::transports::WebSocket;

use std::fs;

use web3::contract::Contract;
use web3::futures::{future, StreamExt};
use web3::types::Address;
use web3::Web3;

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

pub struct Bot {
    web3: web3::Web3<WebSocket>,
    platforms: Vec<Platform>,
    tokens: Vec<Erc20Token<WebSocket>>,
}

impl Bot {
    pub fn new(web3: Web3<WebSocket>, config_file: &str, abi_folder: &str) -> Self {
        let config_data = fs::read_to_string(config_file).expect("Unable to read config file");
        let config: Config = serde_json::from_str(&config_data).unwrap();

        let platforms: Vec<Platform> = Vec::new();
        let tokens: Vec<Erc20Token<WebSocket>> = Vec::new();

        for platform in config.platforms {
            let separator = if abi_folder.ends_with("/") { "" } else { "/" };
            let abi_filepath = format!("{}{}{}.json", abi_folder, separator, &platform.name);
            let abi_bytes_vector = fs::read(abi_filepath).unwrap();
            let abi_bytes_array: &[u8] = &abi_bytes_vector;

            let platform_addr: Address = platform.address.as_str().parse().unwrap();

            let contract = Contract::from_json(web3.eth(), platform_addr, abi_bytes_array).unwrap();

            let dex_platform: Option<Box<dyn Exchange>> = match platform.name.as_str() {
                "kyber" => {
                    let kyber = Kyber;
                    let dex: Box<dyn Exchange> = Box::new(kyber);
                    Some(dex)
                }
                "uniswap" => {
                    let uniswap = Uniswap;
                    let dex: Box<dyn Exchange> = Box::new(uniswap);
                    Some(dex)
                }
                _ => None,
            };

            if let Some(exchange) = dex_platform {
                let platform = Platform { contract, exchange };
                platforms.push(platform);
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
            tokens.push(erc20);
        }

        Bot {
            web3,
            platforms,
            tokens,
        }
    }

    async fn start(&self) -> web3::Result {
        let mut sub = self.web3.eth_subscribe().subscribe_new_heads().await?;

        (&mut sub)
            .take(5)
            .for_each(|x| {
                println!("Got: {:?}", x);
                future::ready(())
            })
            .await;

        sub.unsubscribe().await?;

        Ok(())
    }
}

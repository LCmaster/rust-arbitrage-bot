pub mod platform;
pub mod token;

use self::platform::{Exchange, Kyber, Platform, Uniswap};
use serde::{Deserialize, Serialize};
use web3::transports::WebSocket;

use futures::join;
use std::fs;

use web3::contract::Contract;
use web3::futures::{future, StreamExt};
use web3::types::{Address, U256};
use web3::Web3;

use self::token::Erc20Token;

fn wei_to_eth(wei_val: U256) -> f64 {
    let wei_conv = U256::exp10(18).as_u128() as f64;
    let res = wei_val.as_u128() as f64;
    res / wei_conv
}

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

        let mut platforms: Vec<Platform> = Vec::new();
        let mut tokens: Vec<Erc20Token<WebSocket>> = Vec::new();

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
                let platform = Platform::new(platform.name.as_str(), contract, exchange);
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

    pub async fn start(&self) -> web3::Result {
        let mut sub = self.web3.eth_subscribe().subscribe_new_heads().await?;

        while let Some(_) = (&mut sub).take(5).next().await {
            for platform in &self.platforms {
                let tx1 = async {
                    platform.exchange.get_price_rate(
                        self.tokens[0].contract.address(),
                        self.tokens[1].contract.address(),
                        U256::exp10(18) * 100,
                    )
                };

                let tx2 = async {
                    platform.exchange.get_price_rate(
                        self.tokens[1].contract.address(),
                        self.tokens[0].contract.address(),
                        U256::exp10(18) * 1288,
                    )
                };

                if let (Ok((expected_rate1, _)), Ok((expected_rate2, _))) = join!(tx1, tx2) {
                    let buy_rate = 1.0 / wei_to_eth(expected_rate2);
                    let sell_rate = wei_to_eth(expected_rate1);

                    println!(
                        "{} {}/{}",
                        platform.name,
                        &self.tokens[0].name.to_uppercase(),
                        &self.tokens[1].name.to_uppercase()
                    );
                    println!("Buy: {}; Sell: {}", buy_rate, sell_rate);
                }
            }
        }

        sub.unsubscribe().await?;

        Ok(())
    }
}

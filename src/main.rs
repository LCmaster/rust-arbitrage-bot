use std::env;
use std::fs;
use std::str::FromStr;

use std::collections::HashMap;

use futures::join;

use serde::{Deserialize, Serialize};
use web3::contract::{Contract, Options, Result};
use web3::transports::WebSocket;
use web3::types::{Address, U256};

pub mod bot;
use crate::bot::Bot;

#[derive(Serialize, Deserialize)]
struct DexDescriptor {
    name: String,
    address: String,
}
#[derive(Serialize, Deserialize)]
struct TokenDescriptor {
    name: String,
    address: String,
}

#[derive(Serialize, Deserialize)]
struct Addresses {
    dex: Vec<DexDescriptor>,
    token: Vec<TokenDescriptor>,
}

fn wei_to_eth(wei_val: U256) -> f64 {
    let wei_conv = U256::exp10(18).as_u128() as f64;
    let res = wei_val.as_u128() as f64;
    res / wei_conv
}

async fn get_price_rate(
    dex: &Contract<WebSocket>,
    token1: Address,
    token2: Address,
    amount: U256,
) -> Result<(U256, U256)> {
    dex.query(
        "getExpectedRate",
        (token1, token2, amount),
        None,
        Options::default(),
        None,
    )
    .await
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    let infura_api_key = env::var("INFURA_API_KEY").unwrap();
    let infura_network = env::var("INFURA_MAINNET").unwrap();

    let infura_url = format!("{}{}", &infura_network, &infura_api_key);

    let websocket = web3::transports::WebSocket::new(&infura_url).await?;
    let web3 = web3::Web3::new(websocket);

    let bot = Bot::new(web3, "./addresses.json", "./abi");

    bot.start()

    // let addresses_file = fs::read_to_string("./addresses.json").expect("Couldn't read the file");
    // let addresses: Addresses = serde_json::from_str(&addresses_file).unwrap();

    // let eth_address: Address =
    //     Address::from_str("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap();
    // const EMPTY_HINT: &str = "0x";
    // const PLATFORM_FEE: u32 = 25;
    // const AMOUNT_ETH: u32 = 100;
    // const RECENT_ETH_PRICE: u32 = 1277;
    // let amount_eth_wei: U256 = U256::exp10(18) * U256::from(AMOUNT_ETH);
    // let amount_dai_wei: U256 = U256::exp10(18) * U256::from(RECENT_ETH_PRICE);

    // let mut token_map: HashMap<String, Address> = HashMap::new();

    // for token in addresses.token {
    //     token_map.insert(token.name.to_owned(), token.address.parse().unwrap());
    // }

    // let dai_abi = "./abi/dai.json";
    // let dai_addr = token_map["dai"];
    // let dai_abi_bytes_vector = fs::read(dai_abi).unwrap();

    // let dai_token = Contract::from_json(web3s.eth(), dai_addr, &dai_abi_bytes_vector).unwrap();

    // for dex in addresses.dex {
    //     let dex_abi_filepath = format!("./abi/{}.json", &dex.name);
    //     let dex_abi_bytes_vector = fs::read(dex_abi_filepath).unwrap();
    //     let dex_addr = dex.address.parse().unwrap();
    //     let dex_contract =
    //         Contract::from_json(web3s.eth(), dex_addr, &dex_abi_bytes_vector).unwrap();

    //     let tx1 = get_price_rate(
    //         &dex_contract,
    //         dai_token.address(),
    //         eth_address.to_owned(),
    //         amount_dai_wei.to_owned(),
    //     );
    //     let tx2 = get_price_rate(
    //         &dex_contract,
    //         eth_address.to_owned(),
    //         dai_token.address(),
    //         amount_eth_wei.to_owned(),
    //     );

    //     if let (Ok((dai_expected_rate, _)), Ok((eth_expected_rate, _))) = join!(tx1, tx2) {
    //         let kyber_rate_buy = 1.0 / wei_to_eth(dai_expected_rate);
    //         let kyber_rate_sell = wei_to_eth(eth_expected_rate);

    //         println!("Kyber ETH/DAI");
    //         println!("Buy: {}; Sell: {}", kyber_rate_buy, kyber_rate_sell);
    //     }
    // }

    // kyber::KyberDex::new

    // Ok(())
}

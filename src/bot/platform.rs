use web3::contract::Contract;
use web3::transports::WebSocket;
use web3::types::{Address, U256};

pub trait Exchange {
    fn get_price_rate(&self, token1: Address, token2: Address, ammount: U256);
}

pub struct Kyber;
impl Exchange for Kyber {
    fn get_price_rate(&self, token1: Address, token2: Address, ammount: U256) {
        println!("Kyber Price Rates");
    }
}

pub struct Uniswap;
impl Exchange for Uniswap {
    fn get_price_rate(&self, token1: Address, token2: Address, ammount: U256) {
        println!("Uniswap Price Rates");
    }
}

pub struct DyDx;
impl Exchange for DyDx {
    fn get_price_rate(&self, token1: Address, token2: Address, ammount: U256) {
        println!("DyDx Price Rates");
    }
}

pub struct Platform {
    contract: Contract<WebSocket>,
    exchange: Box<dyn Exchange>,
}
impl Platform {
    fn new(contract: Contract<WebSocket>, exchange: Box<dyn Exchange>) -> Self {
        Self { contract, exchange }
    }
}

use web3::contract::{Contract, Result};
use web3::transports::WebSocket;
use web3::types::{Address, U256};

pub trait Exchange {
    fn get_price_rate(
        &self,
        token1: Address,
        token2: Address,
        ammount: U256,
    ) -> Result<(U256, U256)>;
}

pub struct Kyber;
impl Exchange for Kyber {
    fn get_price_rate(
        &self,
        token1: Address,
        token2: Address,
        ammount: U256,
    ) -> Result<(U256, U256)> {
        Ok((U256::from(1), U256::from(1)))
    }
}

pub struct Uniswap;
impl Exchange for Uniswap {
    fn get_price_rate(
        &self,
        token1: Address,
        token2: Address,
        ammount: U256,
    ) -> Result<(U256, U256)> {
        Ok((U256::from(1), U256::from(1)))
    }
}

pub struct DyDx;
impl Exchange for DyDx {
    fn get_price_rate(
        &self,
        token1: Address,
        token2: Address,
        ammount: U256,
    ) -> Result<(U256, U256)> {
        Ok((U256::from(1), U256::from(1)))
    }
}

pub struct Platform {
    pub name: String,
    pub contract: Contract<WebSocket>,
    pub exchange: Box<dyn Exchange>,
}
impl Platform {
    pub fn new(name: &str, contract: Contract<WebSocket>, exchange: Box<dyn Exchange>) -> Self {
        Self {
            name: name.to_owned(),
            contract,
            exchange,
        }
    }
}

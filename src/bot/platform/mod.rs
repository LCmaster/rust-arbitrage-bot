use web3::types::{Address, U256};

pub mod kyber;
pub mod uniswap;

pub trait DecentralizedExchange {
    fn get_price_rate(token1: Address, token2: Address, ammount: U256);
}

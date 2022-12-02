use web3::contract::Contract;
use web3::types::{Address, U256};
use web3::Transport;

use super::DecentralizedExchange;

pub struct Uniswap<T: Transport> {
    contract: Contract<T>,
}

impl<T: Transport> Uniswap<T> {
    pub fn new(contract: Contract<T>) -> Self {
        Uniswap { contract }
    }
}

impl<T: Transport> DecentralizedExchange for Uniswap<T> {
    fn get_price_rate(token1: Address, token2: Address, ammount: U256) {
        println!("Hello, Uniswap!");
    }
}

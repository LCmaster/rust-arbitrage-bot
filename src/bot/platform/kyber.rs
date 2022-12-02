use web3::contract::Contract;
use web3::types::{Address, U256};
use web3::Transport;

use super::DecentralizedExchange;

pub struct Kyber<T: Transport> {
    contract: Contract<T>,
}

impl<T: Transport> Kyber<T> {
    pub fn new(contract: Contract<T>) -> Self {
        Kyber { contract }
    }
}

impl<T: Transport> DecentralizedExchange for Kyber<T> {
    fn get_price_rate(token1: Address, token2: Address, ammount: U256) {
        println!("Hello, Kyber!");
    }
}

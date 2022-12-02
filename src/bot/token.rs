use web3::contract::Contract;
use web3::Transport;

pub struct Erc20Token<T: Transport> {
    pub name: String,
    pub contract: Contract<T>,
}

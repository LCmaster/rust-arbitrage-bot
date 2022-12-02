use web3::contract::Contract;
use web3::Transport;

pub struct Erc20Token<T: Transport> {
    name: String,
    contract: Contract<T>,
}

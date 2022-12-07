use web3::contract::Contract;
use web3::ethabi::Address;

pub struct Erc20Token {
    pub name: String,
    pub address: Address,
}

impl Erc20Token {
    pub fn new(name: String, address: Address) -> Self {
        Self { name, address }
    }
}

struct TokenPair(Erc20Token, Erc20Token);
impl TokenPair {
    pub fn new(base: Erc20Token, quote: Erc20Token) -> Self {
        Self(base, quote)
    }
}

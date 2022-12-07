use parent::token::{Erc20Token, TokenPair};

pub struct Kyber {
    contract: Contract<WebSocket>,
}

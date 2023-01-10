struct DexPlatform {
    address: String,
    name: String
}

struct ERC20Token {
    address: String,
    symbol: String,
    name: String
}

struct TokenPair(ERC20Token, ERC20Token);

struct TokenPairPrice {
    pair: TokenPair,
    ask: f32,
    bid: f32
}

#[cfg(test)]
mod tests {
    use super::*;
}

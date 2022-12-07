use std::env;

use lib::bot::Bot;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    let infura_api_key = env::var("INFURA_API_KEY").unwrap();
    let infura_network = env::var("INFURA_MAINNET").unwrap();

    let infura_url = format!("{}{}", &infura_network, &infura_api_key);

    let websocket = web3::transports::WebSocket::new(&infura_url).await?;
    let web3 = web3::Web3::new(websocket);

    let bot = Bot::new(web3, "./config.json", "./abi");

    bot.start().await
}

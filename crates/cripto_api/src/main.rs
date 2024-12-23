use cripto_api::{api::CoinService, coins::CoinID};

const API_KEY: &str = "YOUR API KEY";

#[tokio::main]
async fn main() {
    let coin_service = CoinService::new(API_KEY);

    println!(
        "Bitcoin {:?}",
        coin_service.get_coin_price(CoinID::Bitcoin).await
    );
    println!(
        "Solana {:?}",
        coin_service.get_coin_price(CoinID::Solana).await
    );
    println!(
        "Usual {:?}",
        coin_service.get_coin_price(CoinID::Usual).await
    );
    println!("XRP {:?}", coin_service.get_coin_price(CoinID::XRP).await);
    println!("Pepe {:?}", coin_service.get_coin_price(CoinID::Pepe).await);
    println!("Doge {:?}", coin_service.get_coin_price(CoinID::Doge).await);
}

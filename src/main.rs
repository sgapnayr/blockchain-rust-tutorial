#[derive(Debug)] // Derive the Debug trait for LemonadeTransaction

struct LemonadeTransaction {
    id: String,
    buyer_wallet_address: String,
    seller_wallet_address: String,
    amount_of_crypto: f64,
    currency_denomination: String,
    amount_of_lemonade: i128,
    time_of_transaction: String,
}

fn main() {
    let transaction = handle_lemonade_transaction();
    println!("Little Timmy Sold a Lemonade: {:?}", transaction); 
}

fn handle_lemonade_transaction() -> LemonadeTransaction {
    LemonadeTransaction {
        id: "1".to_string(),
        buyer_wallet_address: "0x123".to_string(),
        seller_wallet_address: "0x321".to_string(),
        amount_of_crypto: 0.1,
        currency_denomination: "BTC".to_string(),
        amount_of_lemonade: 1,
        time_of_transaction: "2023-09-12T15:39:00".to_string(),
    }
}

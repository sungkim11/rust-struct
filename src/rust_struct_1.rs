extern crate serde;
use serde::{Serialize, Deserialize};

extern crate reqwest;
use reqwest::blocking::Client;

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinPrice {
    pub base: String,
    pub currency: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinbasePrice {
    pub data: CoinPrice
}

pub fn rust_struct_1() {
    let spot_url = format!("https://api.coinbase.com/v2/prices/{currency}-{rates}/spot",
        currency = "BTC",
        rates = "USD");

    let client = Client::new();
    let resp_spot_price = client.get(&spot_url)
        .send();
    
    match resp_spot_price {
        Ok(parsed_spot_price) => {
            let coinprice = parsed_spot_price.json::<CoinbasePrice>().unwrap();
            
            let spot_price = CoinPrice {
                base: coinprice.data.base,
                currency: coinprice.data.currency,
                amount: coinprice.data.amount
            };

            println!("SPOT: {base}-{currency}: {amount}",
                base=spot_price.base,
                currency=spot_price.currency,
                amount=spot_price.amount);
        }
        Err(e) => println!("Err: {:?}", e),
    }    
}
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

impl CoinPrice {
    fn print_coinprice(self) {
        println!("SPOT: {base}-{currency}: {amount}",
            base=self.base,
            currency=self.currency,
            amount=self.amount);
    }
}

pub fn rust_struct_3() {
    let spot_url = format!("https://api.coinbase.com/v2/prices/{currency}-{rates}/spot",
        currency = "BTC",
        rates = "USD");

    let client = Client::new();     
    let resp_spot_price = client.get(&spot_url)
        .send();

    match resp_spot_price {
        Ok(parsed_spot_price) => {
            let coinbaseprice = parsed_spot_price.json::<CoinbasePrice>()
                .unwrap();            
            let spot_price = CoinPrice {
                base: coinbaseprice.data.base,
                currency: coinbaseprice.data.currency,
                amount: coinbaseprice.data.amount
            };
            spot_price.print_coinprice();            
        }
        Err(e) => println!("Err: {:?}", e),
    }    
}

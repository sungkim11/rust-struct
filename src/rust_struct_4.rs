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

pub trait Price {
    fn format_coinprice(&self) -> String;
    fn return_coinprice(&self) -> String;
}

impl Price for CoinPrice {
    fn format_coinprice(&self) -> String {
        return format!("SPOT: {base}-{currency}: {amount}",
            base=self.base,
            currency=self.currency,
            amount=self.amount);
    }
    fn return_coinprice(&self) -> String {
        return format!("SPOT: {amount}",
            amount=self.amount);
    }
}

pub fn rust_struct_4() {
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
            println!("{}", spot_price.format_coinprice());
            println!("{}", spot_price.return_coinprice());            
        }
        Err(e) => println!("Err: {:?}", e),
    }    
}


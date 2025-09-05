use core::f64;

use tokio_tungstenite::{connect_async,tungstenite::protocol::Message};
use futures_util::{  StreamExt};
use serde::Deserialize;
use url::form_urlencoded::parse;
use std::fmt::Write;
use std::time::Duration;
use kafka::producer::{Producer,Record,RequiredAcks};

#[derive(Debug, Deserialize)]
struct TradeEvent {
    stream:String,
    data:Datas
}

#[derive(Debug, Deserialize)]
struct Datas{
    e: String,   // Event type
    E: u64,      // Event time
    s: String,   // Symbol
    t: u64,      // Trade ID
    p: String,   // Price
    q: String,   // Quantity
    m: bool,
}


#[tokio::main]
async fn main() {
    let url = "wss://stream.binance.com:9443/stream?streams=btcusdt@trade";
    println!("{}", url);
    let (ws_stream, response) =connect_async(url).await.expect("error");
    println!("no connection error");
    let(mut write , mut read)= ws_stream.split();
    while let Some(message)= read.next().await{
        match message {
            Ok(Message::Text(text)) => {
                match serde_json::from_str::<TradeEvent>(&text) {
                    Ok(trade) => {
                        let (buy_price,sell_price)=change_price(trade.data.p.parse().unwrap());
                        println!("{}, {}", buy_price,sell_price)

                    }
                    Err(e) => {
                        println!("Failed to parse trade event: {}", e);
                    }
                }
            }
            Ok(_)=>{},
            Err(e) => println!("error: {}", e),
        }
    }

}
fn change_price(price: f64) -> (f64, f64) {
    let spread_const = 0.005;
    let buy_price = price + (spread_const * price);
    let sell_price = price - (spread_const * price);
    (buy_price, sell_price)
}
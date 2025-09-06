use core::f64;

use futures_util::StreamExt;
use kafka::producer::{self, Producer, Record};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[derive(Debug, Deserialize)]
struct TradeEvent {
    // stream: String,
    data: Datas,
}

#[derive(Debug, Deserialize)]
struct Datas {
    // e: String, // Event type
    // E: u64,    // Event time
    // s: String, // Symbol
    // t: u64,    // Trade ID
    p: String, // Price
    // q: String, // Quantity
    // m: bool,
}
#[derive(Serialize)]
struct Prices {
    buy_price: f64,
    sell_price: f64,
    avg_price: f64,
}

#[tokio::main]
async fn main() {
    let mut producer = match connect_toKafka(){
        Ok(p)=>p,
        Err(e)=>{
            eprintln!("what is this {}",e);
            return ;
        }
    };
    let url = "wss://stream.binance.com:9443/stream?streams=btcusdt@trade";
    println!("{}", url);
    let (ws_stream, _) = connect_async(url).await.expect("error");
    println!("no connection error");
    let (_, mut read) = ws_stream.split();
    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(text)) => match serde_json::from_str::<TradeEvent>(&text) {
                Ok(trade) => {
                    let (buy_price, sell_price, avg_price) =
                        change_price(trade.data.p.parse().unwrap());
                    println!("{}, {}", buy_price, sell_price);
                    let allprices = Prices {
                        buy_price,
                        sell_price,
                        avg_price,
                    };
                    producer.send(&Record::from_value(
                        "axness",
                        serde_json::to_string(&allprices).unwrap(),
                    ));
                }
                Err(e) => {
                    println!("Failed to parse trade event: {}", e);
                }
            },
            Ok(_) => {}
            Err(e) => println!("error: {}", e),
        }
    }
}
fn change_price(price: f64) -> (f64, f64, f64) {
    let spread_const = 0.005;
    let buy_price = price + (spread_const * price);
    let sell_price = price - (spread_const * price);
    let avg_price = (buy_price + sell_price) / 2.0;
    (buy_price, sell_price, avg_price)
}

fn connect_toKafka() -> Result<Producer, kafka::error::Error> {
    Producer::from_hosts(vec!["localhost:9092".to_string()])
        .create()
}

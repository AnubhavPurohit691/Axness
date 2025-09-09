use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use crate::trading_fn::trading_function::Price;
use crate::Inmemory::Inmemory::Trades;
use crate::{Inmemory::Inmemory::User, kafka::kafkaconnect};
use std::sync::{Arc, Mutex, mpsc};
use std::{collections::HashMap, thread};

mod Inmemory;
mod kafka;
mod trading_fn;

fn main() {
    let price_data: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
     let mut users: Vec<User> = Vec::new();
     let mut trades :Vec<Trades>=Vec::new();
     let trade_by_user: HashMap<String, Vec<User>> = HashMap::new();
    let price_data_clone = Arc::clone(&price_data);

     thread::spawn(move || {
        let mut connect2 = match kafka::kafkaconnect::connect_to_kafka(String::from("price")) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };
        loop {
            for ms in connect2.poll().unwrap().iter() {
                for m in ms.messages() {
                    let messagevalue = std::str::from_utf8(m.value).unwrap();
                    let mut price_data_arc = price_data_clone.lock().unwrap();
                    *price_data_arc = Some(messagevalue.to_string());
                }
            }
        }
    });
    // let mainThreadPrice = price_data
    //     .lock()
    //     .unwrap()
    //     .as_ref()
    //     .expect("thread not working")
    //     .clone();

    let mut connect = match kafka::kafkaconnect::connect_to_kafka(String::from("trade")) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    loop {
        for ms in connect.poll().unwrap().iter() {
            for m in ms.messages() {
                let message_key = std::str::from_utf8(m.key).unwrap();
                let message_value = std::str::from_utf8(m.value).unwrap();
                let current_price = {
                let price_guard = price_data.lock().unwrap();
                    price_guard.clone()
                };
                    if let Some(ref price) = current_price {
                        trading_fn::trading_function::trading_function(
                            message_value ,
                            price,
                             &mut trades ,
                             message_key
                        );
                    } else {
                        eprintln!("No price data available for trading.");
                    }
                }
            }
        }
    
}

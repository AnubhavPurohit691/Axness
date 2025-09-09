use crate::Inmemory::Inmemory::{Trades, User};
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Price {
    pub buy_price: Decimal,
    pub sell_price: Decimal,
    pub avg_price: Decimal,
}
pub fn trading_function(m: &str, r: &String,trade: &mut Vec<Trades>, message_key: &str) {
    let t: Trades = serde_json::from_str(m).unwrap();
    let gettingprice: Price = serde_json::from_str(&r).unwrap();
    match message_key {
        "trade" => {
            opentrading_function(t, gettingprice, trade);
        }
        "close" => {
            closetrading_function(t, gettingprice, trade);
        },
        _=>{}
        
    }
}

fn opentrading_function(t: Trades, gettingprice: Price, trades: &mut Vec<Trades>) {
    let open_price= if t.Type == "buy" {
            gettingprice.buy_price
        } else {
            gettingprice.sell_price
        };
    let trade = Trades {
        asset: t.asset,
        openPrice: open_price,
        Type: t.Type,
        tradeId: t.tradeId,
        marginPrice: if let Some(leverage) = t.leverage {
            Some((open_price * t.quantity) / leverage)
        } else {
            None
        },
        quantity: t.quantity,
        takeProfit: t.takeProfit,
        userId: t.userId,
        status: t.status,
        leverage: t.leverage,
        stopLoss: t.stopLoss,
    };
    trades.push(trade);
    println!(" trades is mine {:?}",trades);
}

fn closetrading_function(t: Trades, gettingprice: Price, users: &mut Vec<Trades>) {}

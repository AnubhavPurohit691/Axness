use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub balances: BalanceType,
    pub Id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceType {
    pub amt: i64,
    pub coin: Vec<CoinType>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CoinType {
    pub assert: String,
    pub quantity: Decimal,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Trades {
    pub tradeId: String,
    pub Type:String,
    pub userId: String,
    pub asset: String,
    pub status: String,
    pub openPrice: Decimal,
    pub leverage: Option<Decimal>,
    pub quantity: Decimal,
    pub marginPrice: Option<Decimal>,
    pub stopLoss: Option<Decimal>,
    pub takeProfit: Option<Decimal>,
}

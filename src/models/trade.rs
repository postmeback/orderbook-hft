use std::time::SystemTime;

#[derive(Debug)]
pub struct Trade {
    pub  buy_order_id : String,
    pub  sell_order_id : String,
    pub  price : f64,
    pub  quantity : i32,
    pub  settlement_time: SystemTime
}
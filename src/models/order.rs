use std::time::SystemTime;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub id: String,
    pub price: OrderedFloat<f64>,
    pub quantity: i32,
    pub is_buy: bool,
    pub order_time: SystemTime,
}


mod models {
    pub mod order;
    pub mod trade;
    pub mod order_book;
}

use std::time::SystemTime;
use ordered_float::OrderedFloat;
use models::order::Order;
use models::order_book::OrderBook;

fn main() {
    let mut order_book = OrderBook::new();

    // Adding a buy order
    let buy_order = Order {
        id: "B1".to_string(),
        price: OrderedFloat(100.0),
        quantity: 10,
        is_buy: true,
        order_time: SystemTime::now(),
    };

    // Adding a sell order
    let sell_order = Order {
        id: "S1".to_string(),
        price: OrderedFloat(99.0),
        quantity: 5,
        is_buy: false,
        order_time: SystemTime::now(),
    };

    // Add orders to the order book
    order_book.add_order(buy_order);
    order_book.add_order(sell_order);

    // Print the order book and trades
    order_book.print_order_book();
    order_book.print_trades();
}

use std::collections::{BTreeMap, VecDeque};
use std::time::SystemTime;
use crate::models::order::Order;
use crate::models::trade::Trade;
use ordered_float::OrderedFloat;

pub struct OrderBook {
    pub buy_orders: BTreeMap<OrderedFloat<f64>, VecDeque<Order>>,
    pub sell_orders: BTreeMap<OrderedFloat<f64>, VecDeque<Order>>,
    pub trades : VecDeque<Trade>,
    // pub last_trade_time: SystemTime,
    // pub last_trade_price: f64,
    // pub last_trade_quantity: i32,
    // pub last_trade_id: String,
}

impl OrderBook {
    pub fn new() -> Self{
        OrderBook {
            buy_orders: BTreeMap::new(),
            sell_orders: BTreeMap::new(),
            trades: VecDeque::new(),
            // last_trade_time: SystemTime::now(),
            // last_trade_price: 0.0,
            // last_trade_quantity: 0,
            // last_trade_id: String::new(),
        }
    }

    pub fn add_order(&mut self, mut order: Order) {
        if order.is_buy {
            Self::match_order(&mut self.trades, &mut order, &mut self.sell_orders, true);
            if order.quantity > 0 {
                self.buy_orders
                    .entry(order.price)
                    .or_insert_with(VecDeque::new)
                    .push_back(order);
            }
        } else {
            Self::match_order(&mut self.trades, &mut order, &mut self.buy_orders, false);
            if order.quantity > 0 {
                self.sell_orders
                    .entry(order.price)
                    .or_insert_with(VecDeque::new)
                    .push_back(order);
            }
        }
    }


    fn match_order(trades: &mut VecDeque<Trade>, incoming: &mut Order, book: &mut BTreeMap<OrderedFloat<f64>, VecDeque<Order>>, buy_incoming: bool) {
        let keys: Vec<OrderedFloat<f64>> = if buy_incoming {
            book.keys().filter(|&&k| k <= incoming.price).copied().collect()
        } else {
            book.keys().rev().filter(|&&k| k >= incoming.price).copied().collect()
        };

        for price in keys {
            // Using `match` to handle the `Option` returned by `get_mut`
            match book.get_mut(&price) {
                Some(queue) => {
                    // Proceed with matching orders
                    while let Some(existing_order) = queue.front_mut() {
                        let trade_qty = incoming.quantity.min(existing_order.quantity);
                        existing_order.quantity -= trade_qty;
                        incoming.quantity -= trade_qty;

                        trades.push_back(Trade {
                            buy_order_id: if buy_incoming {
                                incoming.id.clone()
                            } else {
                                existing_order.id.clone()
                            },
                            sell_order_id: if buy_incoming {
                                existing_order.id.clone()
                            } else {
                                incoming.id.clone()
                            },
                            price: price.into_inner(),
                            quantity: trade_qty,
                            settlement_time: SystemTime::now(),
                        });

                        if existing_order.quantity == 0 {
                            queue.pop_front();
                        }
                        if incoming.quantity == 0 {
                            break;
                        }
                    }

                    if queue.is_empty() {
                        book.remove(&price);
                    }
                }
                None => {
                    // If there's no match, we can log it, or do nothing
                    println!("No matching order for price: {}", price);
                }
            }

            if incoming.quantity == 0 {
                break;
            }
        }
    }



    pub fn print_order_book(&self) {
        println!("--- BUY ORDERS ---");
        for (price, orders) in self.buy_orders.iter().rev() {
            for order in orders {
                println!("Buy: ID={}, Qty={}, Price={}", order.id, order.quantity, price);
            }
        }
        println!("--- SELL ORDERS ---");
        for (price, orders) in self.sell_orders.iter() {
            for order in orders {
                println!("Sell: ID={}, Qty={}, Price={}", order.id, order.quantity, price);
            }
        }
    }

    pub fn print_trades(&self) {
        println!("--- TRADES ---");
        for trade in &self.trades {
            let settlement_time = trade.settlement_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            // Printing the trade with the settlement time
            println!(
                "Trade: Buy={}, Sell={}, Price={}, Qty={}, Settlement Time={}",
                trade.buy_order_id,
                trade.sell_order_id,
                trade.price,
                trade.quantity,
                settlement_time
            );
        }
    }

}


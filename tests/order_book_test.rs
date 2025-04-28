#[cfg(test)]
mod tests {
    use std::time::SystemTime;
    use ordered_float::OrderedFloat;
    use orderbook_hft::{Order, OrderBook};

    // Test for a simple trade (one buy order and one sell order)
    #[test]
    fn test_simple_trade() {
        let mut ob = OrderBook::new();
        ob.add_order(Order {
            id: "B1".into(),
            price: OrderedFloat::from(100.0),
            quantity: 5,
            is_buy: true,
            order_time: SystemTime::now(),
        });

        ob.add_order(Order {
            id: "S1".into(),
            price: OrderedFloat::from(99.0),
            quantity: 5,
            is_buy: false,
            order_time: SystemTime::now(),
        });

        assert_eq!(ob.trades.len(), 1);
        assert_eq!(ob.trades[0].quantity, 5);
        assert!(ob.buy_orders.is_empty());
        assert!(ob.sell_orders.is_empty());
    }

    // Test for a partial trade (buy order partially matches a sell order)
    #[test]
    fn test_partial_trade() {
        let mut ob = OrderBook::new();

        ob.add_order(Order {
            id: "B1".into(),
            price: OrderedFloat::from(100.0),
            quantity: 10,
            is_buy: true,
            order_time: SystemTime::now(),
        });

        ob.add_order(Order {
            id: "S1".into(),
            price: OrderedFloat::from(100.0),
            quantity: 5,
            is_buy: false,
            order_time: SystemTime::now(),
        });

        assert_eq!(ob.trades.len(), 1); // One trade happens
        assert_eq!(ob.trades[0].quantity, 5); // The quantity traded is the smaller of the two
        assert_eq!(ob.buy_orders.len(), 1); // The remaining buy order with 5 quantity
        assert_eq!(ob.sell_orders.len(), 0); // The sell order should be removed
    }

    // Test for when buy order is added with larger quantity than sell order
    #[test]
    fn test_buy_order_larger_than_sell_order() {
        let mut ob = OrderBook::new();

        ob.add_order(Order {
            id: "B1".into(),
            price: OrderedFloat::from(100.0),
            quantity: 10,
            is_buy: true,
            order_time: SystemTime::now(),
        });

        ob.add_order(Order {
            id: "S1".into(),
            price: OrderedFloat::from(100.0),
            quantity: 5,
            is_buy: false,
            order_time: SystemTime::now(),
        });

        assert_eq!(ob.trades.len(), 1); // One trade should happen
        assert_eq!(ob.trades[0].quantity, 5); // The smaller quantity is traded
        assert_eq!(ob.buy_orders.len(), 1); // Remaining buy order has 5 quantity
        assert_eq!(ob.sell_orders.len(), 0); // Sell order should be fulfilled
    }

    // Test for an edge case where order quantity is zero
    #[test]
    fn test_zero_quantity_order() {
        let mut ob = OrderBook::new();

        ob.add_order(Order {
            id: "B1".into(),
            price: OrderedFloat::from(100.0),
            quantity: 0,
            is_buy: true,
            order_time: SystemTime::now(),
        });

        ob.add_order(Order {
            id: "S1".into(),
            price: OrderedFloat::from(100.0),
            quantity: 5,
            is_buy: false,
            order_time: SystemTime::now(),
        });

        assert_eq!(ob.trades.len(), 0); // No trade should happen because the buy order has zero quantity
        assert_eq!(ob.buy_orders.len(), 0); // No buy orders should be added
        assert_eq!(ob.sell_orders.len(), 1); // The sell order should still exist
    }
}

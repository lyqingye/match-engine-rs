use std::collections::{BTreeSet, HashMap};
use rust_decimal::Decimal;
use crate::order::{Order, OrderSide, OrderSortKey, OrderType};

#[derive(Debug,Clone)]
pub struct OrderBook {
    pub bids: BTreeSet<OrderSortKey>,
    pub asks: BTreeSet<OrderSortKey>,
    pub orders: HashMap<u64,Order>,
}

impl OrderBook {

    pub fn new() -> OrderBook {
        OrderBook {
            bids: BTreeSet::new(),
            asks: BTreeSet::new(),
            orders: HashMap::with_capacity(1024),
        }
    }

    pub fn place_order(&mut self, order: Order) {
        let key = order.make_sort_key();
        self.orders.insert(order.oid,order);
        match order.side {
            OrderSide::Bid => self.bids.insert(key),
            OrderSide::Ask => self.asks.insert(key),
        };
    }

    pub fn remove_order(& mut self, id: u64) -> Option<Order> {
        let order = self.orders.remove(&id)?;
        let key = order.make_sort_key();
        match order.side {
            OrderSide::Bid => self.bids.remove(&key),
            OrderSide::Ask => self.asks.remove(&key),
        };
        Some(order)
    }
}

#[cfg(test)]
mod test{
    use rust_decimal::Decimal;
    use crate::order::OrderTimeInForce;
    use super::*;

    #[test]
    pub fn test_order_book_sort() {
        let mut book = OrderBook::new();
        for i in 0..10 {
            let mut order = Order::default();
            order.oid = i;
            order.price = Decimal::from(i);
            book.place_order(order);
        }
    }
}
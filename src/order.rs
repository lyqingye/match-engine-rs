use std::cmp::Ordering;
use rust_decimal::Decimal;

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
}

impl OrderType {
    pub fn priority(&self) -> u8 {
        match self {
            OrderType::Market => 0,
            OrderType::Limit => 1,
            OrderType::Stop => 1,
        }
    }
}

impl PartialOrd<Self> for OrderType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority().cmp(&other.priority()).reverse()
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum OrderSide {
    Bid,
    Ask,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum OrderTimeInForce {
    /// Good till cancelled
    GTC,
    /// Immediate closing or cancellation
    IOC,
    /// Fill ok kill
    FOK,
}

#[derive(Debug,Clone,Copy)]
pub struct Order {
    pub oid: u64,
    pub sid: u16,
    pub uid: u32,
    pub price: Decimal,
    pub quantity: Decimal,
    pub balance: Decimal,
    pub t: OrderType,
    pub side: OrderSide,
    pub tif: OrderTimeInForce,
    pub ts: u64,
}

impl Default for Order {
    fn default() -> Self {
        Order {
            oid: 0,
            sid: 0,
            uid: 0,
            price: Default::default(),
            quantity: Default::default(),
            balance: Default::default(),
            t: OrderType::Limit,
            side: OrderSide::Bid,
            tif: OrderTimeInForce::GTC,
            ts: 0
        }
    }
}

impl Order {
    pub fn rollback_for(&mut self,order: &Order) {
        assert_eq!(self.oid,order.oid);
        self.quantity = order.quantity;
        self.balance = order.balance;
    }

    pub fn make_sort_key(&self) -> OrderSortKey {
        OrderSortKey {
            t: self.t,
            side: self.side,
            price: self.price,
            oid: self.oid,
        }
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct OrderSortKey {
    pub t: OrderType,
    pub side: OrderSide,
    pub price: Decimal,
    pub oid: u64,
}


impl PartialOrd<Self> for OrderSortKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderSortKey {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.oid == other.oid {
            return Ordering::Equal;
        }
        match self.t.cmp(&other.t) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                match (self.price.cmp(&other.price),self.side) {
                    (Ordering::Less,OrderSide::Bid) => Ordering::Less,
                    (Ordering::Less,OrderSide::Ask) => Ordering::Greater,
                    (Ordering::Greater,OrderSide::Bid) => Ordering::Greater,
                    (Ordering::Greater,OrderSide::Ask) => Ordering::Less,
                    // compare timestamp
                    (Ordering::Equal,_) => self.oid.cmp(&other.oid).reverse()
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_order_sort_key() {
        let k1 = OrderSortKey {
            t: OrderType::Market,
            side: OrderSide::Bid,
            price: Decimal::from(1),
            oid: 0
        };
        let k2 = OrderSortKey {
            t: OrderType::Market,
            side: OrderSide::Bid,
            price: Decimal::from(2),
            oid: 1
        };
        assert_eq!(Ordering::Less,k1.cmp(&k2));
    }
}


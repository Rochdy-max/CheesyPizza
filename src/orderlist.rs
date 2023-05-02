use crate::order;

#[derive(Debug)]
pub struct OrderList {
    pub orders: std::vec::Vec<order::Order>
}

impl OrderList {
    pub fn new() -> Self {
        Self { orders: vec![] }
    }

    pub fn push_order(&mut self, ord: order::Order) {
        self.orders.push(ord)
    }

    pub fn pop_order(&mut self) -> Option<order::Order> {
        self.orders.pop()
    }
}

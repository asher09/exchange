use std::collections::HashMap;

use crate::input::Side;

pub struct Orderbook {
    pub bids: HashMap<u32, Vec<UserOrder>>,
    pub asks: HashMap<u32, Vec<UserOrder>> ,
}

pub struct UserOrder {
    pub user_id: u32,
    pub qty: u32,
    pub order_id: u32
}

impl Orderbook {
    pub fn new() -> Self {
        Self {
            bids: HashMap::new(),
            asks: Vec::new()
        }
    }
}

impl Orderbook {
    pub fn create_order(&mut self, user_id: u32, qty: u32, price: u32, side: Side) {
        let order_id = self.order_id_index;
        self.order_id_index += 1;
        if(side == Side::Buy) {
            self.bids.entry(price).or_insert(Vec::new()).push(UserOrder {
                user_id,
                qty: quantity,
                order_id: order_id
            });
        } else {
            self.asks.entry(price).or_insert(Vec::new()).push(UserOrder {
                user_id,
                qty: quantity,
                order_id: order_id
            });
        }
    }
    
    //convert this to a more readable binance like struct
    pub fn get_depth(&self) -> Depth {
        self
    }
}




//
//write a function to get the orderbook from the persistence layer
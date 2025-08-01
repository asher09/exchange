use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderInput {
    pub price: f64,
    pub quantity: u32,
    pub user_id: String,
    pub side: Side
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Side {
    Buy, Sell
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteOrderInput {
    pub order_id: String,
}

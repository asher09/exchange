use actix_web::{delete, get, post, web::{Json, Data}, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use crate::{input::{CreateOrderInput, DeleteOrderInput}, orderbook::{self, Orderbook}, output::{CreateOrderResponse, DeleteOrderResponse, Depth}};


#[post("/order")]
pub async fn create_order(body: Json<CreateOrderInput>, orderbook: Data<Arc<Mutex<Orderbook> > >) -> impl Responder {
    let price = body.0.price;
    let quantity = body.0.quantity;
    let user_id = body.0.user_id;
    let side = body.0.side;

    //orderbook logic
    let mut orderbook = orderbook.lock().unwrap();
    orderbook.create_order(price, quantity, user_id, side);

    return HttpResponse::Ok().json(CreateOrderResponse{
        order_id: String::from("order_id")
    });
}

#[delete("/order")]
pub async fn delete_order( Json(body): Json<DeleteOrderInput>, orderbook: Data<Orderbook>) -> impl Responder {
    let order_id = body.order_id;
    HttpResponse::Ok().json(DeleteOrderResponse {
        filled_qty: 0,
        average_price: 100,
    })
}

#[get("/depth")]
pub async fn get_order() -> impl Responder {
    HttpResponse::Ok().json(Depth {
        bids: vec![],
        asks: vec![],
        last_updated: String::from("2023-10-01T12:00")
    })
}



// Additional routes eg:
    // signup
    // signin
    // deposit
    // withdrawal
    // spread
    // currentBal
    // StocksUserhold
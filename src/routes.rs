use actix_web::{delete, get, post, web::Json, HttpResponse, Responder};

use crate::{input::{CreateOrderInput, DeleteOrderInput}, output::{CreateOrderResponse, DeleteOrderResponse, Depth}};


#[post("/order")]
pub async fn create_order(body: Json<CreateOrderInput>) -> impl Responder {
    let _price = body.0.price;
    let _quantity = body.0.quantity;
    let _user_id = body.0.user_id;
    let _side = body.0.side;

    //orderbook logic

    return HttpResponse::Ok().json(CreateOrderResponse{
        order_id: String::from("order_id")
    });
}

#[delete("/order")]
pub async fn delete_order( Json(body): Json<DeleteOrderInput>) -> impl Responder {
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
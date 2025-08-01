use actix_web::{App, HttpServer, Result};

use crate::{orderbook::Orderbook, routes::{create_order, delete_order, get_order}};

pub mod routes;
pub mod input;
pub mod output;
pub mod orderbook;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error>{


    HttpServer::new(move || {
        let orderbook = Arc::new(Mutex::Orderbook::new()); // Arc- becomes accessible to all the threads | Mutex- threads will lock before updating
        App::new()
            .app_data(orderbook.clone)
            .service(create_order)
            .service(delete_order)
            .service(get_order)

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}



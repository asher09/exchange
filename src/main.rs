use actix_web::{App, HttpServer, Result};

use crate::routes::{create_order, delete_order, get_order};

pub mod routes;
pub mod input;
pub mod output;


#[actix_web::main]
async fn main() -> Result<(), std::io::Error>{
    HttpServer::new(move || {
        App::new()
            .service(create_order)
            .service(delete_order)
            .service(get_order)

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}



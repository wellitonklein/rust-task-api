mod services;

use actix_web::{App, HttpServer};
use std::io::Error;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    println!("Server started successfully");

    HttpServer::new(move || App::new().configure(services::config))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

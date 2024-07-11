mod model;
mod schema;
mod services;

use actix_web::{middleware::Logger, web, App, HttpServer};
use std::io::Error;

use dotenv::dotenv;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    pub db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    println!("Server started successfully");

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info")
    }

    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must to be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection DB resolved!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(services::config)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

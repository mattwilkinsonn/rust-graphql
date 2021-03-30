#![deny(warnings)]
mod db;
mod gql;
mod server;
mod utils;

use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use gql::schema::create_schema;
use server::{attach_routes, create_cors, create_redis};
use std::env;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let pool = db::get_db_pool().await;

    let schema = create_schema(pool);

    let server = HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(create_cors())
            .wrap(create_redis())
            .configure(attach_routes)
    });

    server.bind("localhost:4000").unwrap().run().await
}

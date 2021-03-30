#![deny(warnings)]

use std::env;

use actix_cors::Cors;
use actix_web::{guard, http::header, middleware, web, App, HttpResponse, HttpServer};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};
use dotenv::dotenv;
use gql::schema::{create_schema, GQLSchema};
mod db;
mod gql;

async fn graphql_endpoint(schema: web::Data<GQLSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn playground() -> Result<HttpResponse, ()> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
        )))
}

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
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:4000")
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(graphql_endpoint),
            )
            .service(web::resource("/graphql").guard(guard::Get()).to(playground))
    });

    server.bind("localhost:4000").unwrap().run().await
}

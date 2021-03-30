use actix_cors::Cors;
use actix_redis::RedisSession;
use actix_session::Session;
use actix_web::{guard, http::header, web, HttpRequest, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};
use std::env;

use crate::gql::schema::GQLSchema;
pub struct MyToken(String);

async fn graphql_endpoint(
    schema: web::Data<GQLSchema>,
    req: HttpRequest,
    gql_request: Request,
    session: Session,
) -> Response {
    let mut request = gql_request.into_inner();
    request.data(session);

    schema.execute(request).await.into()
}

async fn playground() -> Result<HttpResponse, ()> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
        )))
}

pub fn attach_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/graphql")
            .guard(guard::Post())
            .to(graphql_endpoint),
    )
    .service(web::resource("/graphql").guard(guard::Get()).to(playground));
}

pub fn create_cors() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_methods(vec!["POST", "GET"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .supports_credentials()
        .max_age(3600)
}

pub fn create_redis() -> RedisSession {
    let addr = env::var("REDIS_ADDRESS").expect("Redis address not set");
    let private_key = env::var("SESSION_KEY")
        .expect("Session key not set")
        .as_bytes();
    RedisSession::new(addr, private_key)
}

use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenv::dotenv;
use log::info;

mod application;
mod domain;
mod infrastructure;
mod presentation;

use infrastructure::database;
use presentation::graphql::schema::{create_schema, AppSchema};
use presentation::rest::handlers::health_check;

async fn graphql_handler(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> actix_web::Result<actix_web::HttpResponse> {
    Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
        )))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let db = database::init().await;
    let schema = create_schema(Arc::new(db));

    info!("Starting server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/graphql").to(graphql_handler))
            .service(web::resource("/").to(graphql_playground))
            .route("/health", web::get().to(health_check))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

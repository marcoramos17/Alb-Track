mod db;
mod routes;
mod handlers;
mod types;

use axum::Router;
use db::connector::connect_pool;
use tokio::net::TcpListener;
use axum::serve;
use axum::routing::get_service;
use tower_http::{
    cors::{CorsLayer, Any},
    services::ServeDir
};

#[tokio::main]
async fn main() {
    let pool = connect_pool().await;

    let app = Router::new()
        .nest_service("/photos", get_service(ServeDir::new("backend/data/photos")))
        .merge(routes::routes())
        .with_state(pool)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        );

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}

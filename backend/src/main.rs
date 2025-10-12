use axum::{
    routing::get,
    Router,
    response::Json,
    extract::Extension,
};
use serde_json::{Value, json};
use tower_http::cors::{CorsLayer, Any};
use std::time::Duration;

mod db;
mod structs;

use db::SuviDB;

async fn health_check() -> Json<Value> {
    Json(json!({ "response": "ok" }))
}

async fn set_suvi_handler(
    Extension(db): Extension<SuviDB>
) -> Json<serde_json::Value> {
    match db.increment_suvi().await {
        Ok(_) => Json(json!({ "success": true })),
        Err(_) => Json(json!({ "success": false })),
    }
    
}

async fn get_suvi_handler(
    Extension(db): Extension<SuviDB>
) -> Json<serde_json::Value> {
    let suvi = db.get_suvi().await;
    match suvi {
        Some(s) => Json(json!({ "value": s.value })),
        None => Json(json!({ "value": 0})),
    }
}

#[tokio::main]
async fn main() {

    let uri: String = std::env::var("MONGO_URI").unwrap_or(String::from("mongodb://mongo:27017"));

    let db = db::SuviDB::connect(&uri, "suviplusplus").await;

    println!("Connected to MongoDB");

    let app = Router::new()
        .route("/", get("hello world"))
        .route("/api/health", get(health_check))
        .route("/api/suvi", get(get_suvi_handler).post(set_suvi_handler))
        .layer(Extension(db))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
                .allow_headers(Any)
                .max_age(Duration::from_secs(3600))
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

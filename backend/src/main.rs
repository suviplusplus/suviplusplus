use axum::{
    Router, extract::{ ConnectInfo, Extension, Path }, http::StatusCode, response::{IntoResponse, Json}, routing::get
};
use serde_json::{Value, json};
use tower_http::cors::{CorsLayer, Any};
use std::{net::{SocketAddr}, str::FromStr, time::Duration};
use bson::{DateTime, oid::ObjectId};

use tokio::signal;

mod db;
mod structs;

use db::SuviDB;
use crate::structs::{BlogPost, Comment, NewBlogPost, NewComment, BlogPostResponse, CommentResponse};

async fn health_check() -> Json<Value> {
    Json(json!({ "response": "ok" }))
}

async fn set_suvi_handler(
    Extension(db): Extension<SuviDB>
) -> impl IntoResponse {
    match db.increment_suvi().await {
        Ok(res) => (StatusCode::OK, Json(json!({ "value": res }))).into_response(),
        Err(e) => (StatusCode:: INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
    
}

async fn get_suvi_handler(
    Extension(db): Extension<SuviDB>
) -> impl IntoResponse {
    let suvi = db.get_suvi().await;
    match suvi {
        Some(s) => Json(json!({ "value": s.value })).into_response(),
        None => Json(json!({ "value": 0 })).into_response(),
    }
}

async fn get_blog_handler(
    Extension(db): Extension<SuviDB>
) -> impl IntoResponse {
    let blog = db.get_blog().await;

    match blog {
        Ok(blog) => {
            let response: Vec<BlogPostResponse> =
                blog.into_iter().map(BlogPostResponse::from).collect(); 
            Json(serde_json::to_value(response).unwrap_or_default()).into_response()
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
    
}

async fn get_blog_by_id_handler(
    Path(id): Path<String>,
    Extension(db): Extension<SuviDB>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return (StatusCode::BAD_REQUEST, "Invalid ID").into_response();
    };

    match db.get_blog_post(oid).await {
        Ok(Some(post)) => Json(BlogPostResponse::from(post)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Post not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn set_blog_handler(
    Extension(db): Extension<SuviDB>,
    Json(payload): Json<NewBlogPost>,
) -> impl IntoResponse {
    if !password_check(payload.key) {
        (StatusCode::FORBIDDEN, "invalid key").into_response()
    } else {
        let post = BlogPost {
            id: None,
            title: payload.title,
            body: payload.body,
            date: DateTime::now(),
            comments: Vec::new(),
        };

        match db.insert_blog(post).await {
            Ok(res) => (StatusCode::CREATED, Json(json!({ "id": res.inserted_id }))).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}

async fn get_comments_handler(
    Path(id): Path<String>,
    Extension(db): Extension<SuviDB>,
) -> impl IntoResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return (StatusCode::BAD_REQUEST, "Invalid ID").into_response();
    };

    match db.get_blog_post(oid).await {
        Ok(Some(post)) => {
            let comments: Vec<CommentResponse> = 
                post.comments.into_iter().map(CommentResponse::from).collect();
            Json(comments).into_response()
        },
        Ok(None) => (StatusCode::NOT_FOUND, "Post not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn add_comment_handler(
    Path(id): Path<String>,
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    Extension(db): Extension<SuviDB>,
    Json(payload): Json<NewComment>,
) -> impl IntoResponse {
    let comment = Comment {
        id: Some(ObjectId::new()),
        author: payload.author,
        body: payload.body,
        date: DateTime::now(),
        ip: ip.ip().to_string(),
    };
    
    if let Some(oid) = ObjectId::from_str(&id).ok() {
        match db.add_comment(oid, comment).await {
            Ok(_) => (StatusCode::CREATED, Json(json!({ "id": id }))).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    } else {
        (StatusCode::BAD_REQUEST, "invalid object id").into_response()
    }
}

fn password_check(
    password: String
) -> bool {
    std::env::var("PASSWORD").unwrap_or(String::from("")) == password
}

async fn shutdown_signal() {
    let ctrl_c = async { signal::ctrl_c().await.unwrap() };
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .unwrap().recv().await
    };
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[tokio::main]
async fn main() {

    let uri: String = std::env::var("MONGO_URI").unwrap_or(String::from("mongodb://mongo:27017"));

    let db = SuviDB::connect(&uri, "suviplusplus").await;

    println!("Connected to MongoDB");

    let app = Router::new()
        .route("/", get("hello world"))
        .route("/api/health", get(health_check))
        .route("/api/suvi", get(get_suvi_handler).post(set_suvi_handler))
        .route("/api/blog", get(get_blog_handler).post(set_blog_handler))
        .route("/api/blog/{id}", get(get_blog_by_id_handler))
        .route("/api/blog/{id}/comments", get(get_comments_handler).post(add_comment_handler))
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
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

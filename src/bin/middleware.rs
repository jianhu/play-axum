use anyhow::Result;
use play_axum::{User, AppState};
use std::{sync::{Arc}, time::Duration,};
use parking_lot::RwLock;
use anyhow::Ok;
use hyper::{StatusCode};
use axum::{
    routing::get,
    Router,
    response::Json,
    extract::{State, Path}, error_handling::HandleErrorLayer, BoxError,
};
use tower::{limit::{rate::{RateLimitLayer}, ConcurrencyLimitLayer}, ServiceBuilder, buffer::BufferLayer};

#[tokio::main]
async fn main() -> Result<()> {
    let users = play_axum::prepare_users();
    let user_cache = Arc::new(RwLock::new(users));
    let shared_state = AppState{user_cache};

    let router = Router::new()
        .route("/", get(|| async { "Hello, how can I serve your?" }))
        .route("/user/:id", get(query_user).layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled error: {}", err),
                )}))
                .layer(BufferLayer::new(1024))
                .layer(RateLimitLayer::new(1000, Duration::from_secs(1)))
        ))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled error: {}", err),
                )}))
                .layer(BufferLayer::new(1024))
                .layer(ConcurrencyLimitLayer::new(10))
        )
        .with_state(shared_state.clone());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await?;
    Ok(())
}

// query user, from user cache in `AppState`
pub async fn query_user(
    Path(user_id): Path<i32>,
    State(state): State<AppState>
) -> (StatusCode, Json<Option<User>>) {
     let user_cache = state.user_cache.clone();
     let user_cache = user_cache.read();
     let maybe_user = user_cache.get(&user_id);
     match maybe_user {
        None => {
            return (
                StatusCode::NOT_FOUND, 
                Json(None),
            );
        }
        Some(user) => {
            return (
                StatusCode::OK, 
                Json(Some((*user).clone())),
            );
        }
     }
}
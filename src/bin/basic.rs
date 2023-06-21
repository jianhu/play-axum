use anyhow::Result;
use std::{sync::{Arc}};
use parking_lot::RwLock;
use anyhow::Ok;
use hyper::{StatusCode, Request};
use axum::{
    handler::Handler,
    Extension,
    routing::get,
    Router,
    response::Json,
    extract::{State, Path},
    middleware::{self, Next},
    response::Response,
};
use play_axum::{User, AppState};

#[tokio::main]
async fn main() -> Result<()> {
    let users = play_axum::prepare_users();
    let user_cache = Arc::new(RwLock::new(users));
    let shared_state = AppState{user_cache};
    let numbers = Arc::new(vec![1,2,3]);

    let router = Router::new()
        .route("/", get(|| async { "Hello, how can I serve your?" }))
        .route("/user/:id", get(query_user))
        .route("/user_/:id", get(query_user_))
        .route("/sum", get(sum.layer(Extension(numbers))))
        .route_layer(middleware::from_fn(with_user))
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

// this handler will get user stored in extension
// by `with_user` middleware
pub async fn query_user_(
    Extension(user): Extension<User>
) -> (StatusCode, Json<Option<User>>) {
    (
        StatusCode::OK, 
        Json(Some(user)),
    )
}

// sum handler
pub async fn sum(
    Extension(numbers): Extension<Arc<Vec<i32>>>
) -> String {
    let sum: i32 = numbers.iter().sum();
    sum.to_string()
}

// this middleware inject user into extension
async fn with_user<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Response {
    request.extensions_mut().insert(User{id: 1, name:"".into() , email:"".into(), age: 1});
    let response = next.run(request).await;
    return response;
}
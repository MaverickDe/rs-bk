
use axum::{
    routing::{get,post Router},
    response::Html,
    extract::Path,
    http::StatusCode,
};



pub async fn login() -> &'static str {
    "Hello world!"
}

pub fn create_auth_router() -> Router {
    Router::new()
        .post("/login", get(login))


}


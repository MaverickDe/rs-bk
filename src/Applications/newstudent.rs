
use axum::{
    routing::{get,post Router},
    response::Html,
    extract::Path,
    http::StatusCode,
};



 async fn directentry() -> &'static str {
    "Hello world!"
}
 async fn newstudent() -> &'static str {
    "Hello world!"
}

pub fn create_auth_router() -> Router {
    Router::new()
        .post("/directentry", get(directentry)).post("/new",post(newstudent))


}


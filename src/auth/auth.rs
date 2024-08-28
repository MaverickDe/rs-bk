
use axum::{
    routing::{get,post, Router},
    response::Html,
    extract::Path,
    http::StatusCode,
};






pub async fn login() -> &'static str {
    // let updated: Option<Record> = db
    // .update(("person", "jaime"))
    // .merge(Responsibility { marketing: true })
    // .await?;
    // dbg!(updated);

    
    // println!(db)

    "Hello world!"
}

pub fn create_auth_router() -> Router {
    Router::new()
        .route("/login", get(login))


}


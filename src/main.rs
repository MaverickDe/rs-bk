use axum::{Router, routing::{get}};
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub mod auth;
use auth::auth::{
    create_auth_router
};





async fn hello_world() -> &'static str {
    "Hello world!"
}
// #[tokio::main]
#[tokio::main(flavor = "current_thread")]
async fn main() {
  
 
    let router = Router::new().route("/", get(hello_world))
    .nest("/auth",create_auth_router());
    let addr = SocketAddr::from(([127,0,0,1], 8000));
    let tcp = TcpListener::bind(&addr).await.unwrap();
    axum::serve(tcp, router).await.unwrap();








// static files
//     let router = Router::new()
//     .route_service("/", ServeDir::new("static")
//         .not_found_service(ServeFile::new("static/index.html") 
//      )
// );
}
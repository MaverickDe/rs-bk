use axum::{Router, routing::{get}};
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub mod applications;
pub mod database;
pub mod auth;
use auth::auth::*;
// pub use applications::newstudent::*;
pub use applications::newstudent::*;
pub use database::database::*;





async fn hello_world() -> &'static str {
    let s =String::from("F");
    let c =  testt{first_name:&s};
    println!("{:?}",c);
    "Hello world!"
}
// #[tokio::main]
#[tokio::main(flavor = "current_thread")]
async fn main() {

  
    let router = Router::new().
    route("/", get(hello_world))
    .nest("/auth",create_auth_router())
    .nest("/apply",applicant());
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
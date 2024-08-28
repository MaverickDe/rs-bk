
use axum::{
    routing::{get,post, Router},
    response::Html,
    Json,

    extract::Path,
    http::StatusCode,
};

use crate::{new_student_jamb_y1,testt };







 async fn direct_entry() -> &'static str {
    let s =String::from("F");
    let c =  testt{first_name:&s};
    println!("{:?}",c);
    "Hello world!"
}
//  async fn jamb_applicant_Y1(Json(payload): Json<new_student_jamb_y1>) -> Json<new_student_jamb_y1>  {
//     // println!("{:?}" , x);
//     println!(payload)
//     "Hello world!"
// }

pub fn applicant() -> Router {
    Router::new()
        .route("/directentry", get(direct_entry))
        // .post("/jambapplicant",post(new_student_jamb_y1))


}


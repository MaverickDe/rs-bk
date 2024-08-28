// use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize)]
struct Name<'a> {
    first: &'a str,
    last: &'a str,

}

#[derive(Debug, Serialize)]
struct Person<'a> {
    title: &'a str,
    name: Name<'a>,
    marketing: bool,
}
#[derive(Debug, Serialize)]
struct Student<'a> {
    entrylevel: &'a str,
    currentlevel: &'a str,
    user_id: &'a str,
    first: &'a str,
    last: &'a str,
    
}

#[derive(Debug, Serialize)]
struct Responsibility {
    marketing: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}


pub async fn db() -> surrealdb::Result<Surreal<surrealdb::engine::remote::ws::Client>> {
    // Connect to the server
    let db: Surreal<surrealdb::engine::remote::ws::Client> = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;






    Ok(db)
}
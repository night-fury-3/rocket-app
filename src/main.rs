#[macro_use]
extern crate rocket;

use rocket::serde::json::{json, Value};

#[get("/")]
fn hello() -> Value {
    json!("Hello, world!")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build().mount("/", routes![hello]).launch().await;
}

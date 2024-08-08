#[macro_use]
extern crate rocket;

mod auth;

use auth::BasicAuth;
use rocket::response::status;
use rocket::serde::json::{json, Value};
#[macro_use]
extern crate rocket_sync_db_pools;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
fn get_rustaceans(_auth: BasicAuth, _db: DbConn) -> Value {
    json!([{ "id": 1, "name": "John Doe" }, { "id": 2, "name": "John Doe again" }])
}
#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}
#[post("/rustaceans", format = "json")]
fn create_rustacean(_auth: BasicAuth) -> Value {
    json!({"id": 3, "name": "John Doe", "email": "john@doe.com"})
}
#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}
#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_rustaceans,
                view_rustacean,
                create_rustacean,
                update_rustacean,
                delete_rustacean
            ],
        )
        .register("/", catchers![not_found])
        .attach(DbConn::fairing())
        .launch()
        .await;
}

#[macro_use]
extern crate rocket;

mod auth;
mod models;
mod schema;

use auth::BasicAuth;
use diesel::prelude::*;
use models::{NewRustacean, Rustacean};
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};
use rocket_sync_db_pools::database;
use schema::rustaceans;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|c| {
        let result = rustaceans::table
            .order(rustaceans::id.desc())
            .limit(1000)
            .load::<Rustacean>(c)
            .expect("DB error");
        json!(result)
    })
    .await
}
#[get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> Value {
    db.run(move |c| {
        let rustacean = rustaceans::table
            .find(id)
            .get_result::<Rustacean>(c)
            .expect("Failed retrieveing rutacean row");
        json!(rustacean)
    })
    .await
}
#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(
    _auth: BasicAuth,
    db: DbConn,
    new_rustacean: Json<NewRustacean>,
) -> Value {
    db.run(|c| {
        let result = diesel::insert_into(rustaceans::table)
            .values(new_rustacean.into_inner())
            .execute(c)
            .expect("Failed inserting new rustacean entry");
        json!(result)
    })
    .await
}
#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(
    id: i32,
    _auth: BasicAuth,
    db: DbConn,
    rustacean: Json<Rustacean>,
) -> Value {
    db.run(move |c| {
        let result = diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::email.eq(rustacean.email.to_owned()),
                rustaceans::name.eq(rustacean.name.to_owned()),
            ))
            .execute(c)
            .expect("Failed updating rustacean entry");
        json!(result)
    })
    .await
}
#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> status::NoContent {
    db.run(move |c| {
        diesel::delete(rustaceans::table.find(id))
            .execute(c)
            .expect("Failed deleting rustacean entry");
        status::NoContent
    })
    .await
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

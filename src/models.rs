use super::schema::rustaceans;
use diesel::{prelude::Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable)]
pub struct Rustacean {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "rustaceans"]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

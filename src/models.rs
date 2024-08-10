use super::schema::rustaceans;
use diesel::{prelude::Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct Rustacean {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "rustaceans"]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

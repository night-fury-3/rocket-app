use diesel::Queryable;
use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct Rustacean {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

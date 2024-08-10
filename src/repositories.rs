use diesel::prelude::*;
use diesel::{QueryResult, SqliteConnection};

use crate::models::NewRustacean;
use crate::{models::Rustacean, schema::rustaceans};

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn find(c: &mut SqliteConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result::<Rustacean>(c)
    }

    pub fn find_multiple(c: &mut SqliteConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.limit(limit).load::<Rustacean>(c)
    }

    pub fn create(c: &mut SqliteConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .execute(c)?;

        let last_id = Self::last_inserted_id(c)?;
        Self::find(c, last_id)
    }

    pub fn save(c: &mut SqliteConnection, id: i32, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::email.eq(rustacean.email.to_owned()),
                rustaceans::name.eq(rustacean.name.to_owned()),
            ))
            .execute(c)?;

        Self::find(c, id)
    }

    pub fn delete(c: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(c)
    }

    fn last_inserted_id(c: &mut SqliteConnection) -> QueryResult<i32> {
        rustaceans::table
            .select(rustaceans::id)
            .order(rustaceans::id.desc())
            .first(c)
    }
}

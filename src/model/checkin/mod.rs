mod common;

use super::error::Error;
use super::schema::checkin::dsl;
use crate::connector::Connectors;
use diesel::prelude::*;
use uuid::Uuid;

pub use common::*;

pub fn insert(connectors: &Connectors, checkin: &CheckinInsert) -> Result<Uuid, Error> {
    let connection = connectors.local.pool.get()?;

    // Insert user if not exists, otherwise update its email which the unhashed version of the login
    diesel::insert_into(dsl::checkin)
        .values(checkin)
        .returning(dsl::id)
        .get_result(&connection)
        .map_err(|error| error.into())
}

pub fn confirm(connectors: &Connectors, session_id: &Uuid) -> Result<(), Error> {
    let connection = connectors.local.pool.get()?;

    diesel::update(dsl::checkin.filter(dsl::session_id.eq(session_id)))
        .set(dsl::confirmed.eq(true))
        .execute(&connection)
        .map(|_| ())
        .map_err(|error| error.into())
}

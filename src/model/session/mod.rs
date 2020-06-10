mod common;

use super::error::Error;
use super::schema::session::dsl;
use crate::connector::Connectors;
use diesel::prelude::*;

pub use common::*;

pub fn insert(connector: &Connectors, session: &SessionInsert) -> Result<Session, Error> {
    let connection = connector.local.pool.get()?;

    // Insert user if not exists, otherwise update its email which the unhashed version of the login
    diesel::insert_into(dsl::session)
        .values(session)
        .get_result(&connection)
        .map_err(|error| error.into())
}

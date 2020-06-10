mod common;

use super::error::Error;
use super::schema::organization::dsl;
use crate::connector::Connectors;
use diesel::prelude::*;

pub use common::*;

pub fn upsert(connectors: &Connectors, org: &OrganizationUpsert) -> Result<(), Error> {
    let connection = connectors.local.pool.get()?;

    // Insert user if not exists, otherwise update its email which the unhashed version of the login
    diesel::insert_into(dsl::organization)
        .values(org)
        .on_conflict(dsl::user_id)
        .do_nothing()
        .execute(&connection)
        .map(|_| ())
        .map_err(|error| error.into())
}

mod common;

use super::error::Error;
use super::schema::organization::dsl;
use crate::connector::Connectors;
use diesel::prelude::*;
use uuid::Uuid;

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

pub fn confirm(connectors: &Connectors, id: &Uuid) -> Result<(), Error> {
    let connection = connectors.local.pool.get()?;

    diesel::update(dsl::organization.find(id))
        .set(dsl::confirmed.eq(true))
        .execute(&connection)
        .map_err(|error| error.into())
        .and_then(|count| {
            if count == 1 {
                Ok(())
            } else {
                Err(Error::NotFound)
            }
        })
}

mod common;

use super::error::{is_one, Error};
use super::schema::organization::dsl;
use crate::connector::Connector;
use diesel::prelude::*;
use uuid::Uuid;

pub use common::*;

pub fn upsert(connector: &Connector, org: &OrganizationUpsert) -> Result<(), Error> {
    let connection = connector.local.pool.get()?;

    // Insert user if not exists, otherwise update its email which the unhashed version of the login
    diesel::insert_into(dsl::organization)
        .values(org)
        .on_conflict(dsl::user_id)
        .do_nothing()
        .execute(&connection)
        .map(|_| ())
        .map_err(|error| error.into())
}

pub fn set_name(connector: &Connector, id: &Uuid, name: &str) -> Result<(), Error> {
    let connection = connector.local.pool.get()?;

    diesel::update(dsl::organization.find(id))
        .set(dsl::name.eq(name))
        .execute(&connection)
        .map_err(|error| error.into())
        .and_then(|count| is_one(count, "Organization"))
}

pub fn confirm(connector: &Connector, id: &Uuid) -> Result<(), Error> {
    let connection = connector.local.pool.get()?;

    diesel::update(dsl::organization.find(id))
        .set(dsl::confirmed.eq(true))
        .execute(&connection)
        .map(|_| ())
        .map_err(|error| error.into())
}

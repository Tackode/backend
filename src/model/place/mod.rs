mod common;

use super::error::Error;
use super::schema::place::dsl;
use crate::connector::Connectors;
use diesel::prelude::*;
use uuid::Uuid;

pub use common::*;

pub fn get(connectors: &Connectors, id: &Uuid) -> Result<Place, Error> {
    let connection = connectors.local.pool.get()?;

    dsl::place
        .find(id)
        .first::<Place>(&connection)
        .map_err(|error| error.into())
}

pub fn insert(connectors: &Connectors, place: &PlaceInsert) -> Result<(), Error> {
    let connection = connectors.local.pool.get()?;

    diesel::insert_into(dsl::place)
        .values(place)
        .execute(&connection)
        .map(|_| ())
        .map_err(|error| error.into())
}

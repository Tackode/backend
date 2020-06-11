mod common;

use super::error::Error;
use super::organization::Organization;
use super::schema::{organization, place::dsl};
use crate::connector::Connectors;
use diesel::prelude::*;
use uuid::Uuid;

pub use common::*;

pub fn get_with_organization(
    connectors: &Connectors,
    id: &Uuid,
) -> Result<(Place, Organization), Error> {
    let connection = connectors.local.pool.get()?;

    dsl::place
        .inner_join(organization::dsl::organization)
        .filter(dsl::id.eq(id).and(dsl::disabled.eq(false)))
        .first::<(Place, Organization)>(&connection)
        .map_err(|error| error.into())
}

pub fn get_all_with_organization(
    connectors: &Connectors,
    organization_id: &Uuid,
) -> Result<Vec<(Place, Organization)>, Error> {
    let connection = connectors.local.pool.get()?;

    dsl::place
        .inner_join(organization::dsl::organization)
        .filter(
            organization::dsl::id
                .eq(organization_id)
                .and(dsl::disabled.eq(false)),
        )
        .load::<(Place, Organization)>(&connection)
        .map_err(|error| error.into())
}

pub fn insert(connectors: &Connectors, place: &PlaceInsert) -> Result<Uuid, Error> {
    let connection = connectors.local.pool.get()?;

    diesel::insert_into(dsl::place)
        .values(place)
        .returning(dsl::id)
        .get_result(&connection)
        .map_err(|error| error.into())
}

pub fn update(
    connectors: &Connectors,
    id: &Uuid,
    organization_id: &Uuid,
    place: &PlaceUpdate,
) -> Result<(), Error> {
    let connection = connectors.local.pool.get()?;

    diesel::update(dsl::place.filter(dsl::id.eq(id).and(dsl::organization_id.eq(organization_id))))
        .set(place)
        .execute(&connection)
        .map(|_| ())
        .map_err(|error| error.into())
}

pub fn set_disabled(
    connectors: &Connectors,
    id: &Uuid,
    organization_id: &Uuid,
    disabled: bool,
) -> Result<(), Error> {
    let connection = connectors.local.pool.get()?;

    diesel::update(dsl::place.filter(dsl::id.eq(id).and(dsl::organization_id.eq(organization_id))))
        .set(dsl::disabled.eq(disabled))
        .execute(&connection)
        .map(|_| ())
        .map_err(|error| error.into())
}

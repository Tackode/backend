mod common;

use super::error::{is_one, Error};
use super::organization::Organization;
use super::schema::{organization, place::dsl};
use crate::connector::Connector;
use diesel::prelude::*;
use uuid::Uuid;

pub use common::*;

pub fn get(connector: &Connector, id: &Uuid) -> Result<Place, Error> {
    let connection = connector.local.pool.get()?;

    dsl::place
        .filter(dsl::id.eq(id).and(dsl::disabled.eq(false)))
        .first::<Place>(&connection)
        .map_err(|error| error.into())
}

pub fn get_with_organization(
    connector: &Connector,
    id: &Uuid,
) -> Result<(Place, Organization), Error> {
    let connection = connector.local.pool.get()?;

    dsl::place
        .inner_join(organization::dsl::organization)
        .filter(dsl::id.eq(id).and(dsl::disabled.eq(false)))
        .first::<(Place, Organization)>(&connection)
        .map_err(|error| error.into())
}

pub fn get_all_with_organization(
    connector: &Connector,
    organization_id: &Uuid,
) -> Result<Vec<(Place, Organization)>, Error> {
    let connection = connector.local.pool.get()?;

    dsl::place
        .inner_join(organization::dsl::organization)
        .filter(
            organization::dsl::id
                .eq(organization_id)
                .and(dsl::disabled.eq(false)),
        )
        .order(dsl::created_at.desc())
        .load::<(Place, Organization)>(&connection)
        .map_err(|error| error.into())
}

pub fn validate_places_owned(
    connector: &Connector,
    organization_id: &Uuid,
    places_ids: &Vec<Uuid>,
) -> Result<(), Error> {
    let connection = connector.local.pool.get()?;
    let length = places_ids.len() as i64;

    dsl::place
        .select(diesel::dsl::count(dsl::id))
        .filter(
            dsl::organization_id
                .eq(organization_id)
                .and(dsl::id.eq_any(places_ids)),
        )
        .first(&connection)
        .map_err(|error| error.into())
        .and_then(|count: i64| {
            if count == length {
                Ok(())
            } else {
                Err(Error::NotFoundWithName {
                    name: String::from("Place"),
                })
            }
        })
}

pub fn insert(connector: &Connector, place: &PlaceInsert) -> Result<Uuid, Error> {
    let connection = connector.local.pool.get()?;

    diesel::insert_into(dsl::place)
        .values(place)
        .returning(dsl::id)
        .get_result(&connection)
        .map_err(|error| error.into())
}

pub fn update(
    connector: &Connector,
    id: &Uuid,
    organization_id: &Uuid,
    place: &PlaceUpdate,
) -> Result<(), Error> {
    let connection = connector.local.pool.get()?;

    diesel::update(
        dsl::place.filter(
            dsl::id
                .eq(id)
                .and(dsl::organization_id.eq(organization_id))
                .and(dsl::disabled.eq(false)),
        ),
    )
    .set(place)
    .execute(&connection)
    .map_err(|error| error.into())
    .and_then(|count| is_one(count, "Place"))
}

pub fn set_disabled(
    connector: &Connector,
    id: &Uuid,
    organization_id: &Uuid,
    disabled: bool,
) -> Result<(), Error> {
    let connection = connector.local.pool.get()?;

    diesel::update(
        dsl::place.filter(
            dsl::id
                .eq(id)
                .and(dsl::organization_id.eq(organization_id))
                .and(dsl::disabled.eq(false)),
        ),
    )
    .set(dsl::disabled.eq(disabled))
    .execute(&connection)
    .map_err(|error| error.into())
    .and_then(|count| is_one(count, "Place"))
}

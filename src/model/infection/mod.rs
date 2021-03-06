mod common;

use super::checkin;
use super::error::Error;
use super::organization::Organization;
use super::schema::{infection::dsl, organization};
use crate::connector::Connector;
use diesel::prelude::*;
use uuid::Uuid;

pub use common::*;

pub fn get_all_with_organization(
    connector: &Connector,
    organization_id: &Uuid,
) -> Result<Vec<(Infection, Organization)>, Error> {
    let connection = connector.local.pool.get()?;

    dsl::infection
        .inner_join(organization::dsl::organization)
        .filter(organization::dsl::id.eq(organization_id))
        .order(dsl::start_timestamp.desc())
        .load::<(Infection, Organization)>(&connection)
        .map_err(|error| error.into())
}

pub fn get_with_organization(
    connector: &Connector,
    infection_id: &Uuid,
) -> Result<(Infection, Organization), Error> {
    let connection = connector.local.pool.get()?;

    dsl::infection
        .inner_join(organization::dsl::organization)
        .filter(dsl::id.eq(infection_id))
        .first::<(Infection, Organization)>(&connection)
        .map_err(|error| error.into())
}

pub fn insert(connector: &Connector, infection: &InfectionInsert) -> Result<Uuid, Error> {
    let connection = connector.local.pool.get()?;

    let id = diesel::insert_into(dsl::infection)
        .values(infection)
        .returning(dsl::id)
        .get_result(&connection)?;

    checkin::enable_potential_infections(
        connector,
        &infection.places_ids,
        &infection.start_timestamp,
        &infection.end_timestamp,
    )?;

    Ok(id)
}

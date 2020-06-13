mod common;

use super::checkin;
use super::error::Error;
use super::organization::Organization;
use super::schema::{infection::dsl, organization};
use crate::connector::Connectors;
use diesel::prelude::*;
use uuid::Uuid;

pub use common::*;

pub fn get_all_with_organization(
    connectors: &Connectors,
    organization_id: &Uuid,
) -> Result<Vec<(Infection, Organization)>, Error> {
    let connection = connectors.local.pool.get()?;

    dsl::infection
        .inner_join(organization::dsl::organization)
        .filter(organization::dsl::id.eq(organization_id))
        .load::<(Infection, Organization)>(&connection)
        .map_err(|error| error.into())
}

pub fn insert(connectors: &Connectors, infection: &InfectionInsert) -> Result<Infection, Error> {
    let connection = connectors.local.pool.get()?;

    let new_infection = diesel::insert_into(dsl::infection)
        .values(infection)
        .get_result::<Infection>(&connection)?;

    checkin::enable_potential_infections(
        connectors,
        &infection.places_ids,
        &infection.start_timestamp,
        &infection.end_timestamp,
    )?;

    Ok(new_infection)
}

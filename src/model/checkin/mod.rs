mod common;

use super::error::Error;
use super::organization::Organization;
use super::place::Place;
use super::schema::checkin::dsl;
use super::schema::{organization, place};
use crate::connector::Connector;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

pub use common::*;

pub fn get_all_with_user(
    connector: &Connector,
    user_id: &Uuid,
) -> Result<Vec<(Checkin, (Place, Organization))>, Error> {
    let connection = connector.local.pool.get()?;

    dsl::checkin
        .inner_join(place::dsl::place.inner_join(organization::dsl::organization))
        .filter(dsl::user_id.eq(user_id).and(dsl::confirmed.eq(true)))
        .load::<(Checkin, (Place, Organization))>(&connection)
        .map_err(|error| error.into())
}

pub fn insert(connector: &Connector, checkin: &CheckinInsert) -> Result<Uuid, Error> {
    let connection = connector.local.pool.get()?;

    // Insert user if not exists, otherwise update its email which the unhashed version of the login
    diesel::insert_into(dsl::checkin)
        .values(checkin)
        .returning(dsl::id)
        .get_result(&connection)
        .map_err(|error| error.into())
}

pub fn confirm(connector: &Connector, session_id: &Uuid) -> Result<(), Error> {
    let connection = connector.local.pool.get()?;

    diesel::update(dsl::checkin.filter(dsl::session_id.eq(session_id)))
        .set(dsl::confirmed.eq(true))
        .execute(&connection)
        .map(|_| ())
        .map_err(|error| error.into())
}

pub fn enable_potential_infections(
    connector: &Connector,
    places_ids: &Vec<Uuid>,
    start_timestamp: &DateTime<Utc>,
    end_timestamp: &DateTime<Utc>,
) -> Result<(), Error> {
    let connection = connector.local.pool.get()?;

    diesel::update(
        dsl::checkin.filter(
            dsl::place_id
                .eq_any(places_ids)
                .and(dsl::start_timestamp.le(end_timestamp))
                .and(dsl::end_timestamp.ge(start_timestamp)),
        ),
    )
    .set(dsl::potential_infection.eq(true))
    .execute(&connection)
    .map(|_| ())
    .map_err(|error| error.into())
}

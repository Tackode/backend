mod common;

use super::error::Error;
use super::schema::session::dsl;
use crate::connector::Connectors;
use diesel::prelude::*;
use uuid::Uuid;

pub use common::*;

pub fn get_unconfirmed(
    connectors: &Connectors,
    id: &Uuid,
    hashed_confirmation_token: &String,
) -> Result<Session, Error> {
    let connection = connectors.local.pool.get()?;

    dsl::session
        .filter(
            dsl::id
                .eq(id)
                .and(dsl::hashed_confirmation_token.eq(hashed_confirmation_token))
                .and(dsl::confirmed.eq(false))
                .and(dsl::disabled.eq(false)),
        )
        .first::<Session>(&connection)
        .map_err(|error| error.into())
}

pub fn get_confirmed(
    connectors: &Connectors,
    id: &Uuid,
    hashed_token: &String,
) -> Result<Option<Session>, Error> {
    let connection = connectors.local.pool.get()?;

    dsl::session
        .filter(
            dsl::id
                .eq(id)
                .and(dsl::hashed_token.eq(hashed_token))
                .and(dsl::confirmed.eq(true))
                .and(dsl::disabled.eq(false)),
        )
        .first::<Session>(&connection)
        .optional()
        .map_err(|error| error.into())
}

pub fn confirm(connectors: &Connectors, id: &Uuid, hashed_token: &String) -> Result<(), Error> {
    let connection = connectors.local.pool.get()?;

    diesel::update(dsl::session.find(id))
        .set(&SessionTokenUpdate {
            hashed_confirmation_token: None,
            hashed_token: Some(hashed_token.clone()),
            confirmed: true,
        })
        .execute(&connection)
        .map(|_| ())
        .map_err(|error| error.into())
}

pub fn insert(connectors: &Connectors, session: &SessionInsert) -> Result<Session, Error> {
    let connection = connectors.local.pool.get()?;

    // Insert user if not exists, otherwise update its email which the unhashed version of the login
    diesel::insert_into(dsl::session)
        .values(session)
        .get_result(&connection)
        .map_err(|error| error.into())
}

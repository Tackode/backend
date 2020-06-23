mod common;

use super::error::{is_one, Error};
use super::schema::{session::dsl, user};
use super::user::User;
use crate::connector::Connector;
use diesel::prelude::*;
use uuid::Uuid;

pub use common::*;

pub fn get_unconfirmed(
    connector: &Connector,
    id: &Uuid,
    hashed_confirmation_token: &String,
) -> Result<(Session, User), Error> {
    let connection = connector.local.pool.get()?;

    dsl::session
        .inner_join(user::dsl::user)
        .filter(
            dsl::id
                .eq(id)
                .and(dsl::hashed_confirmation_token.eq(hashed_confirmation_token))
                .and(dsl::confirmed.eq(false))
                .and(dsl::disabled.eq(false)),
        )
        .first::<(Session, User)>(&connection)
        .map_err(|error| error.into())
}

pub fn get_confirmed(
    connector: &Connector,
    id: &Uuid,
    hashed_token: &String,
) -> Result<Option<Session>, Error> {
    let connection = connector.local.pool.get()?;

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

pub fn confirm(connector: &Connector, id: &Uuid, hashed_token: &String) -> Result<(), Error> {
    let connection = connector.local.pool.get()?;

    diesel::update(dsl::session.find(id))
        .set(&SessionTokenUpdate {
            hashed_confirmation_token: None,
            hashed_token: Some(hashed_token.clone()),
            confirmed: true,
        })
        .execute(&connection)
        .map_err(|error| error.into())
        .and_then(|count| is_one(count, "Session"))
}

pub fn insert(connector: &Connector, session: &SessionInsert) -> Result<Session, Error> {
    let connection = connector.local.pool.get()?;

    // Insert user if not exists, otherwise update its email which the unhashed version of the login
    diesel::insert_into(dsl::session)
        .values(session)
        .get_result(&connection)
        .map_err(|error| error.into())
}

pub fn set_disabled(connector: &Connector, id: &Uuid, disabled: bool) -> Result<(), Error> {
    let connection = connector.local.pool.get()?;

    // Insert user if not exists, otherwise update its email which the unhashed version of the login
    diesel::update(dsl::session.find(id))
        .set(dsl::disabled.eq(disabled))
        .execute(&connection)
        .map_err(|error| error.into())
        .and_then(|count| is_one(count, "Session"))
}

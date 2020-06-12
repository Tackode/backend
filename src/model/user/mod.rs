mod common;

use super::error::Error;
use super::organization::Organization;
use super::schema::{organization, user::dsl};
use crate::connector::Connectors;
use diesel::prelude::*;
use uuid::Uuid;

pub use common::*;

pub fn get(connectors: &Connectors, id: &Uuid) -> Result<User, Error> {
    let connection = connectors.local.pool.get()?;

    dsl::user
        .filter(dsl::id.eq(id).and(dsl::disabled.eq(false)))
        .first::<User>(&connection)
        .map_err(|error| error.into())
}

pub fn get_with_login(connectors: &Connectors, login: &String) -> Result<User, Error> {
    let connection = connectors.local.pool.get()?;

    dsl::user
        .filter(dsl::login.eq(login).and(dsl::disabled.eq(false)))
        .first::<User>(&connection)
        .map_err(|error| error.into())
}

pub fn get_with_organization(
    connectors: &Connectors,
    id: &Uuid,
) -> Result<(User, Option<Organization>), Error> {
    let connection = connectors.local.pool.get()?;

    dsl::user
        .left_join(organization::dsl::organization)
        .filter(dsl::id.eq(id).and(dsl::disabled.eq(false)))
        .first::<(User, Option<Organization>)>(&connection)
        .map_err(|error| error.into())
}

pub fn insert(
    connectors: &Connectors,
    user: &UserInsert,
    update_email: bool,
) -> Result<User, Error> {
    let connection = connectors.local.pool.get()?;

    // Insert user if not exists
    let insert_count = diesel::insert_into(dsl::user)
        .values(user)
        .on_conflict(dsl::login)
        .do_nothing()
        .execute(&connection)?;

    if insert_count == 0 && update_email {
        set_email(connectors, &user.login, &user.email)?;
    }

    get_with_login(connectors, &user.login)
}

pub fn update_role(connectors: &Connectors, id: Uuid, role: UserRole) -> Result<(), Error> {
    let connection = connectors.local.pool.get()?;

    diesel::update(dsl::user.filter(dsl::id.eq(id).and(dsl::disabled.eq(false))))
        .set(dsl::role.eq(role))
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

pub fn confirm(connectors: &Connectors, id: &Uuid) -> Result<(), Error> {
    let connection = connectors.local.pool.get()?;

    diesel::update(dsl::user.filter(dsl::id.eq(id).and(dsl::disabled.eq(false))))
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

pub fn set_email(
    connectors: &Connectors,
    login: &String,
    email: &Option<String>,
) -> Result<(), Error> {
    let connection = connectors.local.pool.get()?;

    diesel::update(dsl::user.filter(dsl::login.eq(login).and(dsl::disabled.eq(false))))
        .set(dsl::email.eq(email))
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

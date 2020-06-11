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
        .find(id)
        .first::<User>(&connection)
        .map_err(|error| error.into())
}

pub fn get_with_login(connectors: &Connectors, login: &String) -> Result<User, Error> {
    let connection = connectors.local.pool.get()?;

    dsl::user
        .filter(dsl::login.eq(login))
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
        .filter(dsl::id.eq(id))
        .first::<(User, Option<Organization>)>(&connection)
        .map_err(|error| error.into())
}

pub fn insert(connectors: &Connectors, user: &UserInsert) -> Result<User, Error> {
    let connection = connectors.local.pool.get()?;

    // Insert user if not exists
    diesel::insert_into(dsl::user)
        .values(user)
        .on_conflict(dsl::login)
        .do_nothing()
        .execute(&connection)?;

    get_with_login(connectors, &user.login)
}

pub fn upsert(connectors: &Connectors, user: &UserUpsert) -> Result<User, Error> {
    let connection = connectors.local.pool.get()?;

    // Insert user if not exists, otherwise update its email which is the unhashed version of the login
    diesel::insert_into(dsl::user)
        .values(user)
        .on_conflict(dsl::login)
        .do_update()
        .set(dsl::email.eq(user.email.clone()))
        .get_result(&connection)
        .map_err(|error| error.into())
}

pub fn update_role(connectors: &Connectors, id: Uuid, role: UserRole) -> Result<(), Error> {
    let connection = connectors.local.pool.get()?;

    diesel::update(dsl::user.filter(dsl::id.eq(id)))
        .set(dsl::role.eq(role))
        .execute(&connection)
        .map(|_| ())
        .map_err(|error| error.into())
}

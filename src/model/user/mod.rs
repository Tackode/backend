mod common;

use super::error::{is_one, Error};
use super::organization::Organization;
use super::schema::{organization, user::dsl};
use crate::connector::Connector;
use diesel::prelude::*;
use uuid::Uuid;

pub use common::*;

pub fn get(connector: &Connector, id: &Uuid) -> Result<User, Error> {
    let connection = connector.local.pool.get()?;

    dsl::user
        .filter(dsl::id.eq(id).and(dsl::disabled.eq(false)))
        .first::<User>(&connection)
        .map_err(|error| error.into())
}

pub fn exist_with_login(connector: &Connector, login: &String) -> Result<bool, Error> {
    let connection = connector.local.pool.get()?;

    dsl::user
        .select(diesel::dsl::count(dsl::id))
        .filter(dsl::login.eq(login).and(dsl::disabled.eq(false)))
        .first(&connection)
        .map_err(|error| error.into())
        .map(|count: i64| count > 0)
}

pub fn get_with_login(connector: &Connector, login: &String) -> Result<User, Error> {
    let connection = connector.local.pool.get()?;

    dsl::user
        .filter(dsl::login.eq(login).and(dsl::disabled.eq(false)))
        .first::<User>(&connection)
        .map_err(|error| error.into())
}

pub fn get_with_organization(
    connector: &Connector,
    id: &Uuid,
) -> Result<(User, Option<Organization>), Error> {
    let connection = connector.local.pool.get()?;

    dsl::user
        .left_join(organization::dsl::organization)
        .filter(dsl::id.eq(id).and(dsl::disabled.eq(false)))
        .first::<(User, Option<Organization>)>(&connection)
        .map_err(|error| error.into())
}

pub fn insert(connector: &Connector, user: &UserInsert, update_email: bool) -> Result<User, Error> {
    let connection = connector.local.pool.get()?;

    // Insert user if not exists
    let insert_count = diesel::insert_into(dsl::user)
        .values(user)
        .on_conflict(dsl::login)
        .do_nothing()
        .execute(&connection)?;

    if insert_count == 0 && update_email {
        set_email_with_login(connector, &user.login, &user.email)?;
    }

    get_with_login(connector, &user.login)
}

pub fn update_role(connector: &Connector, id: Uuid, role: UserRole) -> Result<(), Error> {
    let connection = connector.local.pool.get()?;

    diesel::update(dsl::user.filter(dsl::id.eq(id).and(dsl::disabled.eq(false))))
        .set(dsl::role.eq(role))
        .execute(&connection)
        .map_err(|error| error.into())
        .and_then(|count| is_one(count, "User"))
}

pub fn confirm(connector: &Connector, id: &Uuid) -> Result<(), Error> {
    let connection = connector.local.pool.get()?;

    diesel::update(dsl::user.filter(dsl::id.eq(id).and(dsl::disabled.eq(false))))
        .set(dsl::confirmed.eq(true))
        .execute(&connection)
        .map_err(|error| error.into())
        .and_then(|count| is_one(count, "User"))
}

pub fn set_email_with_login(
    connector: &Connector,
    login: &String,
    email: &Option<String>,
) -> Result<(), Error> {
    let connection = connector.local.pool.get()?;

    diesel::update(dsl::user.filter(dsl::login.eq(login).and(dsl::disabled.eq(false))))
        .set(dsl::email.eq(email))
        .execute(&connection)
        .map_err(|error| error.into())
        .and_then(|count| is_one(count, "User"))
}

pub fn set_email(connector: &Connector, id: &Uuid, email: &Option<String>) -> Result<(), Error> {
    let connection = connector.local.pool.get()?;

    diesel::update(dsl::user.filter(dsl::id.eq(id).and(dsl::disabled.eq(false))))
        .set(dsl::email.eq(email))
        .execute(&connection)
        .map_err(|error| error.into())
        .and_then(|count| is_one(count, "User"))
}

pub fn delete(connector: &Connector, id: &Uuid) -> Result<(), Error> {
    let connection = connector.local.pool.get()?;

    diesel::delete(dsl::user.find(id))
        .execute(&connection)
        .map_err(|error| error.into())
        .and_then(|count| is_one(count, "User"))
}

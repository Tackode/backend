mod common;

use super::error::Error;
use super::schema::user::dsl;
use crate::connector::Connectors;
use diesel::prelude::*;
use uuid::Uuid;

pub use common::*;

pub fn get(connector: &Connectors, login: &String) -> Result<User, Error> {
    let connection = connector.local.pool.get()?;

    dsl::user
        .filter(dsl::login.eq(login))
        .first::<User>(&connection)
        .map_err(|error| error.into())
}

pub fn insert(connector: &Connectors, user: &UserInsert) -> Result<User, Error> {
    let connection = connector.local.pool.get()?;

    // Insert user if not exists
    diesel::insert_into(dsl::user)
        .values(user)
        .on_conflict(dsl::login)
        .do_nothing()
        .execute(&connection)?;

    get(connector, &user.login)
}

pub fn upsert(connector: &Connectors, user: &UserUpsert) -> Result<User, Error> {
    let connection = connector.local.pool.get()?;

    // Insert user if not exists, otherwise update its email which is the unhashed version of the login
    diesel::insert_into(dsl::user)
        .values(user)
        .on_conflict(dsl::login)
        .do_update()
        .set(dsl::email.eq(user.email.clone()))
        .get_result(&connection)
        .map_err(|error| error.into())
}

pub fn update_role(connector: &Connectors, id: Uuid, role: UserRole) -> Result<(), Error> {
    let connection = connector.local.pool.get()?;

    diesel::update(dsl::user)
        .set(dsl::role.eq(role))
        .filter(dsl::id.eq(id))
        .execute(&connection)
        .map(|_| ())
        .map_err(|error| error.into())
}

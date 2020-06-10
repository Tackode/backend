use super::super::schema::user;
use chrono::{DateTime, Utc};
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};
use std::io::Write;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "Text"]
pub enum UserRole {
    Public,
    Professional,
}

#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub login: String,
    pub email: Option<String>,
    pub role: UserRole,
    pub confirmed: bool,
    pub disabled: bool,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct UserInsert {
    pub login: String,
    pub role: UserRole,
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct UserUpsert {
    pub login: String,
    pub email: Option<String>,
    pub role: UserRole,
}

// SQL conversion
impl ToSql<Text, Pg> for UserRole {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            UserRole::Public => out.write_all(b"public")?,
            UserRole::Professional => out.write_all(b"professional")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for UserRole {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"public" => Ok(UserRole::Public),
            b"professional" => Ok(UserRole::Professional),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

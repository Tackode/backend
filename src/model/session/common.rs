use super::super::schema::session;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Queryable)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub description: String,
    pub hashed_token: Option<String>,
    pub hashed_confirmation_token: Option<String>,
    pub confirmed: bool,
    pub disabled: bool,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "session"]
pub struct SessionInsert {
    pub user_id: Uuid,
    pub description: String,
    pub hashed_confirmation_token: String,
}

#[derive(AsChangeset)]
#[table_name = "session"]
pub struct SessionTokenUpdate {
    pub hashed_confirmation_token: Option<String>,
    pub hashed_token: Option<String>,
    pub confirmed: bool,
}

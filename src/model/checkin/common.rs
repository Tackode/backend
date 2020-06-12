use super::super::schema::checkin;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Queryable)]
pub struct Checkin {
    pub id: Uuid,
    pub place_id: Uuid,
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub start_timestamp: DateTime<Utc>,
    pub end_timestamp: DateTime<Utc>,
    pub duration: i64,
    pub potential_infection: bool,
    pub confirmed: bool,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "checkin"]
pub struct CheckinInsert {
    pub place_id: Uuid,
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub start_timestamp: DateTime<Utc>,
    pub end_timestamp: DateTime<Utc>,
    pub duration: i64,
    pub confirmed: bool,
}

use super::super::schema::infection;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Queryable)]
pub struct Infection {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub places_ids: Vec<Uuid>,
    pub start_timestamp: DateTime<Utc>,
    pub end_timestamp: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "infection"]
pub struct InfectionInsert {
    pub organization_id: Uuid,
    pub places_ids: Vec<Uuid>,
    pub start_timestamp: DateTime<Utc>,
    pub end_timestamp: DateTime<Utc>,
}

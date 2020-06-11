use super::super::schema::place;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Queryable)]
pub struct Place {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub average_duration: i64,
    pub disabled: bool,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "place"]
pub struct PlaceInsert {
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub average_duration: i64,
}

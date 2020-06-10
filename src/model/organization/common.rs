use super::super::schema::organization;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Queryable)]
pub struct Organization {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub confirmed: bool,
    pub disabled: bool,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "organization"]
pub struct OrganizationUpsert {
    pub user_id: Uuid,
    pub name: String,
    pub confirmed: bool,
}

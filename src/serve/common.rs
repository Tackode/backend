use crate::connectors::ConnectorsBuilders;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub const CONTENT_LENGTH_LIMIT: u64 = 1024 * 16;

#[derive(Clone)]
pub struct Context {
    pub builders: ConnectorsBuilders,
}

pub struct PublicUser {
    pub id: Uuid,
}

pub struct ProfessionalUser {
    pub id: Uuid,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub healthy: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanQuery {
    pub place_id: Uuid,
}

#[derive(Serialize)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Place {
    pub id: Uuid,
    pub organization: Organization,
    pub name: String,
    pub description: Option<String>,
    /// Average duration in minutes
    pub average_duration: i64,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlaceForm {
    #[validate(length(min = 1, max = 60))]
    pub name: String,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    /// Average duration in minutes
    #[validate(range(min = 1, max = 480))]
    pub average_duration: i64,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CheckinForm {
    pub place_id: Uuid,
    #[validate(email)]
    pub email: String,
    pub store_email: bool,
    #[validate(range(min = 1, max = 480))]
    pub duration: i64,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValidateSessionForm {
    #[validate(length(equal = 64))]
    pub confirmation_token: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub session_id: Uuid,
}

#[derive(Serialize)]
pub struct Credentials {
    pub login: String,
    pub token: String,
}

#[derive(Serialize)]
pub struct Profile {
    pub id: Uuid,
    pub email: Option<String>,
    pub organization: Option<Organization>,
}

#[derive(Deserialize, Validate)]
pub struct ProfileForm {
    #[validate(email)]
    pub email: Option<String>,
}

#[derive(Deserialize)]
pub struct OrganizationForm {
    pub name: String,
}

#[derive(Serialize)]
pub struct Checkin {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub duration: i64,
    pub place: Place,
}

#[derive(Serialize, Deserialize)]
pub enum UserRole {
    Public,
    Professional,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginForm {
    #[validate(email)]
    pub email: String,
    pub role: UserRole,
    #[validate(length(min = 1, max = 60))]
    pub organization_name: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfectionForm {
    pub places_ids: Vec<Uuid>,
    pub start_timestamp: DateTime<Utc>,
    pub end_timestamp: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Infection {
    pub id: Uuid,
    pub places: Vec<Place>,
    pub start_timestamp: DateTime<Utc>,
    pub end_timestamp: DateTime<Utc>,
}

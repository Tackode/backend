use super::types::*;
use crate::connector::ConnectorsBuilders;
use crate::model::user::UserRole;
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
    pub user: User,
    pub session: Session,
}

pub struct ProfessionalUser {
    pub user: User,
    pub session: Session,
    pub organization: Organization,
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

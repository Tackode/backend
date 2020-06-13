use super::Place;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

use super::Organization;
use crate::model::infection::Infection as InfectionModel;
use crate::model::organization::Organization as OrganizationModel;
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
    pub organization: Organization,
    pub places_ids: Vec<Uuid>,
    pub start_timestamp: DateTime<Utc>,
    pub end_timestamp: DateTime<Utc>,
}

impl From<(InfectionModel, OrganizationModel)> for Infection {
    fn from((infection, organization): (InfectionModel, OrganizationModel)) -> Self {
        Infection {
            id: infection.id,
            organization: organization.into(),
            places_ids: infection.places_ids,
            start_timestamp: infection.start_timestamp,
            end_timestamp: infection.end_timestamp,
        }
    }
}

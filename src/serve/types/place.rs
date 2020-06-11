use super::Organization;
use crate::model::organization::Organization as OrganizationModel;
use crate::model::place::Place as PlaceModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

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

impl From<(PlaceModel, OrganizationModel)> for Place {
    fn from((place, org): (PlaceModel, OrganizationModel)) -> Self {
        Place {
            id: place.id,
            organization: org.into(),
            name: place.name,
            description: place.description,
            average_duration: place.average_duration,
        }
    }
}

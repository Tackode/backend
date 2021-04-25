use super::Organization;
use crate::model::organization::Organization as OrganizationModel;
use crate::model::place::Place as PlaceModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Place {
    pub id: Uuid,
    pub organization: Organization,
    pub name: String,
    pub description: Option<String>,
    /// Average duration in minutes
    pub average_duration: i32,
    pub maximum_gauge: Option<i32>,
    pub address: Option<String>,
    pub location: Option<(f64, f64)>,
    pub maximum_duration: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Gauge {
    pub place_id: Uuid,
    pub value: i64,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlaceForm {
    #[validate(length(min = 1, max = 60))]
    pub name: String,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    /// Average duration in minutes
    #[validate(range(min = 1, max = 1440))]
    pub average_duration: i32,
    #[validate(range(min = 1))]
    pub maximum_gauge: Option<i32>,
    #[validate(length(max = 1000))]
    pub address: Option<String>,
    #[validate(range(min = -90, max = 90))]
    pub latitude: Option<f64>,
    #[validate(range(min = -180, max = 180))]
    pub longitude: Option<f64>,
    #[validate(range(min = 1, max = 1440))]
    pub maximum_duration: i32,
}

impl From<(PlaceModel, OrganizationModel)> for Place {
    fn from((place, org): (PlaceModel, OrganizationModel)) -> Self {
        Place {
            id: place.id,
            organization: org.into(),
            name: place.name,
            description: place.description,
            average_duration: place.average_duration,
            maximum_gauge: place.maximum_gauge,
            address: place.address,
            location: match (place.latitude, place.longitude) {
                (Some(latitude), Some(longitude)) => Some((latitude, longitude)),
                _ => None,
            },
            maximum_duration: place.maximum_duration,
        }
    }
}

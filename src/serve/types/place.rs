use super::Organization;
use crate::model::organization::Organization as OrganizationModel;
use crate::model::place::{Place as PlaceModel, PlaceSearchResult as PlaceSearchResultModel};
use crate::model::types::GaugeLevel as GaugeLevelModel;
use crate::types::{Location, Pagination, PaginationQuery};
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
    pub average_duration: i64,
    pub address: Option<String>,
    pub location: Option<Location>,
    pub current_gauge_level: GaugeLevel,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnedPlace {
    pub id: Uuid,
    pub organization: Organization,
    pub name: String,
    pub description: Option<String>,
    /// Average duration in minutes
    pub average_duration: i64,
    pub maximum_gauge: Option<i64>,
    pub address: Option<String>,
    pub location: Option<Location>,
    pub maximum_duration: i64,
    pub current_gauge: i64,
    pub current_gauge_percent: Option<i64>,
    pub current_gauge_level: GaugeLevel,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GaugeLevel {
    Unknown,
    Safe,
    Warning,
    Alert,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceSearchResult {
    pub meter_distance: f64,
    pub place: Place,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlacesSearchResults {
    pub places: Vec<PlaceSearchResult>,
    pub pagination: Pagination,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlaceSearchQuery {
    pub location: Location,
    #[validate(range(min = 1, max = 1000000))]
    pub radius: i64,
    pub pagination: PaginationQuery,
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
    pub average_duration: i64,
    #[validate(range(min = 1, max = 1000000))]
    pub maximum_gauge: Option<i64>,
    #[validate(length(max = 1000))]
    pub address: Option<String>,
    pub location: Option<Location>,
    #[validate(range(min = 1, max = 1440))]
    pub maximum_duration: i64,
}

impl From<(PlaceModel, OrganizationModel)> for Place {
    fn from((place, org): (PlaceModel, OrganizationModel)) -> Self {
        Place {
            id: place.id,
            organization: org.into(),
            name: place.name,
            description: place.description,
            average_duration: place.average_duration,
            address: place.address,
            location: place.location.map(|point| point.into()),
            current_gauge_level: place.current_gauge_level.into(),
        }
    }
}

impl From<GaugeLevelModel> for GaugeLevel {
    fn from(gauge_level: GaugeLevelModel) -> Self {
        match gauge_level {
            GaugeLevelModel::Alert => GaugeLevel::Alert,
            GaugeLevelModel::Warning => GaugeLevel::Warning,
            GaugeLevelModel::Safe => GaugeLevel::Safe,
            GaugeLevelModel::Unknown => GaugeLevel::Unknown,
        }
    }
}

impl From<(PlaceModel, OrganizationModel)> for OwnedPlace {
    fn from((place, org): (PlaceModel, OrganizationModel)) -> Self {
        OwnedPlace {
            id: place.id,
            organization: org.into(),
            name: place.name,
            description: place.description,
            average_duration: place.average_duration,
            maximum_gauge: place.maximum_gauge,
            address: place.address,
            location: place.location.map(|point| point.into()),
            maximum_duration: place.maximum_duration,
            current_gauge: place.current_gauge,
            current_gauge_percent: place.current_gauge_percent,
            current_gauge_level: place.current_gauge_level.into(),
        }
    }
}

impl From<PlaceSearchResultModel> for PlaceSearchResult {
    fn from(search_result: PlaceSearchResultModel) -> Self {
        PlaceSearchResult {
            meter_distance: search_result.meter_distance,
            place: (search_result.place, search_result.organization).into(),
        }
    }
}

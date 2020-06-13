use super::Place;
use crate::model::checkin::Checkin as CheckinModel;
use crate::model::organization::Organization as OrganizationModel;
use crate::model::place::Place as PlaceModel;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

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

#[derive(Serialize)]
pub struct Checkin {
    pub id: Uuid,
    pub start_timestamp: DateTime<Utc>,
    pub end_timestamp: DateTime<Utc>,
    pub duration: i64,
    pub potential_infection: bool,
    pub place: Place,
}

impl From<(CheckinModel, (PlaceModel, OrganizationModel))> for Checkin {
    fn from((checkin, place_org): (CheckinModel, (PlaceModel, OrganizationModel))) -> Self {
        Checkin {
            id: checkin.id,
            start_timestamp: checkin.start_timestamp,
            end_timestamp: checkin.end_timestamp,
            duration: checkin.duration,
            potential_infection: checkin.potential_infection,
            place: place_org.into(),
        }
    }
}

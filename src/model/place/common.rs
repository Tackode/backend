use crate::model::organization::Organization;

use super::super::schema::place;
use chrono::{DateTime, Utc};
use postgis::ewkb::Point;

use diesel::sql_types::*;
use postgis_diesel::sql_types::*;
use postgis_diesel::*;

#[derive(Queryable)]
pub struct Place {
    pub id: uuid::Uuid,
    pub organization_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub average_duration: i64,
    pub disabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub maximum_gauge: Option<i64>,
    pub address: Option<String>,
    pub maximum_duration: i64,
    pub current_gauge: i64,
    pub location: Option<PointC<Point>>,
}

pub struct PlaceSearchResult {
    pub meter_distance: f64,
    pub place: Place,
    pub organization: Organization,
}

#[derive(Insertable)]
#[table_name = "place"]
pub struct PlaceInsert {
    pub organization_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub average_duration: i64,
    pub maximum_gauge: Option<i64>,
    pub address: Option<String>,
    pub maximum_duration: i64,
    pub location: Option<PointC<Point>>,
}

#[derive(AsChangeset)]
#[table_name = "place"]
#[changeset_options(treat_none_as_null = "true")]
pub struct PlaceUpdate {
    pub name: String,
    pub description: Option<String>,
    pub average_duration: i64,
    pub maximum_gauge: Option<i64>,
    pub address: Option<String>,
    pub maximum_duration: i64,
    pub location: Option<PointC<Point>>,
}

#[derive(QueryableByName)]
pub struct PlaceSearchRow {
    // place table
    #[sql_type = "Uuid"]
    pub id: uuid::Uuid,
    #[sql_type = "Uuid"]
    pub organization_id: uuid::Uuid,
    #[sql_type = "Text"]
    pub name: String,
    #[sql_type = "Nullable<Text>"]
    pub description: Option<String>,
    #[sql_type = "Int8"]
    pub average_duration: i64,
    #[sql_type = "Bool"]
    pub disabled: bool,
    #[sql_type = "Timestamptz"]
    pub created_at: DateTime<Utc>,
    #[sql_type = "Timestamptz"]
    pub updated_at: DateTime<Utc>,
    #[sql_type = "Nullable<Int8>"]
    pub maximum_gauge: Option<i64>,
    #[sql_type = "Nullable<Text>"]
    pub address: Option<String>,
    #[sql_type = "Int8"]
    pub maximum_duration: i64,
    #[sql_type = "Int8"]
    pub current_gauge: i64,
    #[sql_type = "Nullable<Geometry>"]
    pub location: Option<PointC<Point>>,

    // organization table
    #[sql_type = "Uuid"]
    pub org_id: uuid::Uuid,
    #[sql_type = "Uuid"]
    pub org_user_id: uuid::Uuid,
    #[sql_type = "Text"]
    pub org_name: String,
    #[sql_type = "Bool"]
    pub org_confirmed: bool,
    #[sql_type = "Bool"]
    pub org_disabled: bool,
    #[sql_type = "Timestamptz"]
    pub org_updated_at: DateTime<Utc>,
    #[sql_type = "Timestamptz"]
    pub org_created_at: DateTime<Utc>,

    // distance
    #[sql_type = "Float8"]
    pub meter_distance: f64,
}

impl From<PlaceSearchRow> for PlaceSearchResult {
    fn from(place_row: PlaceSearchRow) -> Self {
        PlaceSearchResult {
            meter_distance: place_row.meter_distance,
            place: Place {
                id: place_row.id,
                organization_id: place_row.organization_id,
                name: place_row.name,
                description: place_row.description,
                average_duration: place_row.average_duration,
                disabled: place_row.disabled,
                created_at: place_row.created_at,
                updated_at: place_row.updated_at,
                maximum_gauge: place_row.maximum_gauge,
                address: place_row.address,
                maximum_duration: place_row.maximum_duration,
                current_gauge: place_row.current_gauge,
                location: place_row.location,
            },
            organization: Organization {
                id: place_row.org_id,
                user_id: place_row.org_user_id,
                name: place_row.org_name,
                confirmed: place_row.org_confirmed,
                disabled: place_row.org_disabled,
                updated_at: place_row.org_updated_at,
                created_at: place_row.org_created_at,
            },
        }
    }
}

use postgis::ewkb::Point;
use postgis_diesel::PointC;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    #[validate(range(min = -90, max = 90))]
    pub latitude: f64,
    #[validate(range(min = -180, max = 180))]
    pub longitude: f64,
}

impl From<PointC<Point>> for Location {
    fn from(point: PointC<Point>) -> Self {
        Location {
            latitude: point.v.x,
            longitude: point.v.y,
        }
    }
}

impl From<Location> for PointC<Point> {
    fn from(location: Location) -> Self {
        PointC {
            v: Point {
                srid: Some(4326),
                x: location.latitude,
                y: location.longitude,
            },
        }
    }
}

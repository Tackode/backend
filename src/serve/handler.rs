use super::common::*;
use warp::reply::Reply;

pub fn index() -> impl Reply {
    warp::reply::json(&HealthResponse { healthy: true })
}

pub fn scan(query: ScanQuery, context: Context) -> impl Reply {
    warp::reply::json(&Place {
        id: query.place_id,
        organization: Organization {
            id: query.place_id,
            name: String::from("Creatiwity"),
        },
        name: String::from("Bureau"),
        description: None,
        average_duration: 480,
    })
}

pub fn checkin(data: CheckinForm, context: Context) -> impl Reply {
    warp::reply()
}

pub fn validate_device(data: ValidateDeviceForm, context: Context) -> impl Reply {
    warp::reply::json(&Credentials {
        login: String::from("LOGIN"),
        token: String::from("TOKEN"),
    })
}

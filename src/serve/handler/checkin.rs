use super::super::common::*;
use warp::reply::Reply;

pub fn create(data: CheckinForm, context: Context) -> impl Reply {
    // TODO return deviceId
    warp::reply()
}

pub fn get_all(user: PublicUser, context: Context) -> impl Reply {
    warp::reply::json(&vec![Checkin {
        id: user.id,
        timestamp: chrono::Utc::now(),
        duration: 60,
        place: Place {
            id: user.id,
            organization: Organization {
                id: user.id,
                name: String::from("Creatiwity"),
            },
            name: String::from("Bureau 1"),
            description: None,
            average_duration: 600,
        },
    }])
}

use super::super::common::*;
use uuid::Uuid;
use warp::reply::Reply;

pub fn create(data: CheckinForm, context: Context) -> impl Reply {
    warp::reply::json(&Session {
        session_id: Uuid::parse_str("85f520d0-193d-4386-bdf6-902bc7a4350e").unwrap(),
    })
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

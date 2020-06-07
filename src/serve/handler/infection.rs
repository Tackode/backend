use super::super::common::*;
use uuid::Uuid;
use warp::reply::Reply;

pub fn create(user: ProfessionalUser, data: InfectionForm, context: Context) -> impl Reply {
    warp::reply()
}

pub fn get_all(user: ProfessionalUser, context: Context) -> impl Reply {
    let placeholder_id = Uuid::parse_str("85f520d0-193d-4386-bdf6-902bc7a4350e").unwrap();

    warp::reply::json(&vec![Infection {
        id: placeholder_id,
        start_timestamp: chrono::Utc::now(),
        end_timestamp: chrono::Utc::now(),
        places: vec![Place {
            id: user.id,
            organization: Organization {
                id: user.id,
                name: String::from("Creatiwity"),
            },
            name: String::from("Bureau 1"),
            description: None,
            average_duration: 600,
        }],
    }])
}

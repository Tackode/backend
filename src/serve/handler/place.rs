use super::super::common::*;
use uuid::Uuid;
use warp::reply::Reply;

pub fn get_one(place_id: Uuid, context: Context) -> impl Reply {
    warp::reply::json(&Place {
        id: place_id,
        organization: Organization {
            id: place_id,
            name: String::from("Creatiwity"),
        },
        name: String::from("Bureau"),
        description: None,
        average_duration: 480,
    })
}

pub fn get_all(user: ProfessionalUser, context: Context) -> impl Reply {
    let place_id = Uuid::parse_str("85f520d0-193d-4386-bdf6-902bc7a4350e").unwrap();

    warp::reply::json(&vec![Place {
        id: place_id,
        organization: Organization {
            id: place_id,
            name: String::from("Creatiwity"),
        },
        name: String::from("Bureau"),
        description: None,
        average_duration: 480,
    }])
}

pub fn create(user: ProfessionalUser, data: PlaceForm, context: Context) -> impl Reply {
    warp::reply()
}

pub fn update(
    place_id: Uuid,
    user: ProfessionalUser,
    data: PlaceForm,
    context: Context,
) -> impl Reply {
    warp::reply()
}

pub fn delete(place_id: Uuid, user: ProfessionalUser, context: Context) -> impl Reply {
    warp::reply()
}

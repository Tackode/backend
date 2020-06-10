use super::super::authorization::professional_user_filter;
use super::super::common::*;
use super::super::types::*;
use uuid::Uuid;
use warp::{filters::BoxedFilter, Filter, Reply};

pub fn routes(context: Context) -> BoxedFilter<(impl Reply,)> {
    let moved_context = context.clone();
    let context_filter = warp::any().map(move || moved_context.clone());

    // GET /place/<id> -> Place
    let get_place = warp::get()
        .and(warp::path!("place" / Uuid))
        .and(context_filter.clone())
        .map(get_one);

    // GET /places -> Vec<Place>
    let get_places = warp::get()
        .and(warp::path!("places"))
        .and(professional_user_filter(context.clone()))
        .and(context_filter.clone())
        .map(get_all);

    // POST /place/<id> -> Place
    let create_place = warp::post()
        .and(warp::path!("place"))
        .and(professional_user_filter(context.clone()))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .map(create);

    // POST /place/<id> -> Place
    let set_place = warp::post()
        .and(warp::path!("place" / Uuid))
        .and(professional_user_filter(context.clone()))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .map(update);

    // DELETE /place/<id> -> 200
    let delete_place = warp::delete()
        .and(warp::path!("place" / Uuid))
        .and(professional_user_filter(context.clone()))
        .and(context_filter.clone())
        .map(delete);

    get_place
        .or(get_places)
        .or(create_place)
        .or(set_place)
        .or(delete_place)
        .boxed()
}

fn get_one(place_id: Uuid, context: Context) -> impl Reply {
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

fn get_all(user: ProfessionalUser, context: Context) -> impl Reply {
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

fn create(user: ProfessionalUser, data: PlaceForm, context: Context) -> impl Reply {
    warp::reply()
}

fn update(place_id: Uuid, user: ProfessionalUser, data: PlaceForm, context: Context) -> impl Reply {
    warp::reply()
}

fn delete(place_id: Uuid, user: ProfessionalUser, context: Context) -> impl Reply {
    warp::reply()
}

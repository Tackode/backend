use super::super::authorization::public_user_filter;
use super::super::common::*;
use super::super::types::*;
use uuid::Uuid;
use warp::{filters::BoxedFilter, Filter, Reply};

pub fn routes(context: Context) -> BoxedFilter<(impl Reply,)> {
    let moved_context = context.clone();
    let context_filter = warp::any().map(move || moved_context.clone());

    // POST /checkin {uuid, email, store_email, duration} -> 200
    let checkin = warp::post()
        .and(warp::path!("checkin"))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .map(create);

    // GET /checkins -> Checkin(Place)
    let get_checkins = warp::get()
        .and(warp::path!("checkins"))
        .and(public_user_filter(context.clone()))
        .and(context_filter.clone())
        .map(get_all);

    checkin.or(get_checkins).boxed()
}

fn create(data: CheckinForm, context: Context) -> impl Reply {
    warp::reply::json(&Session {
        id: Uuid::parse_str("85f520d0-193d-4386-bdf6-902bc7a4350e").unwrap(),
    })
}

fn get_all(user: PublicUser, context: Context) -> impl Reply {
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

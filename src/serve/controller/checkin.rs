use super::super::authorization::public_user_filter;
use super::super::common::*;
use super::super::types::*;
use uuid::Uuid;
use warp::{filters::BoxedFilter, Filter, Reply};

pub fn routes(context: Context) -> BoxedFilter<(impl Reply,)> {
    let moved_context = context.clone();
    let context_filter = warp::any().map(move || moved_context.clone());

    // POST /checkin with Auth {uuid, email, store_email, duration} -> 200
    let checkin_with_user = warp::post()
        .and(warp::path!("checkin"))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(public_user_filter(context.clone()))
        .and(context_filter.clone())
        .map(create_with_user);

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

    checkin_with_user.or(checkin).or(get_checkins).boxed()
}

fn create_with_user(data: CheckinForm, public: PublicUser, context: Context) -> impl Reply {
    // Validate data
    // Retrieve place
    // Update user email with id, login and email
    // Create checkin
    warp::reply::json(&Session {
        id: Uuid::parse_str("85f520d0-193d-4386-bdf6-902bc7a4350e").unwrap(),
        confirmed: true,
    })
}

fn create(data: CheckinForm, context: Context) -> impl Reply {
    // Validate data
    // Retrieve place
    // Upsert public user
    // Create session
    // Create checkin
    warp::reply::json(&Session {
        id: Uuid::parse_str("85f520d0-193d-4386-bdf6-902bc7a4350e").unwrap(),
        confirmed: false,
    })
}

fn get_all(public: PublicUser, context: Context) -> impl Reply {
    warp::reply::json(&vec![Checkin {
        id: public.user.id,
        timestamp: chrono::Utc::now(),
        duration: 60,
        place: Place {
            id: public.user.id,
            organization: Organization {
                id: public.user.id,
                name: String::from("Creatiwity"),
            },
            name: String::from("Bureau 1"),
            description: None,
            average_duration: 600,
        },
    }])
}

// TODO: delete all for user

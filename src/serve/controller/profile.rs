use super::super::authorization::public_user_filter;
use super::super::common::*;
use warp::{filters::BoxedFilter, Filter, Reply};

pub fn routes(context: Context) -> BoxedFilter<(impl Reply,)> {
    let moved_context = context.clone();
    let context_filter = warp::any().map(move || moved_context.clone());

    // GET /profile -> Profile(Organization)
    let get_profile = warp::get()
        .and(warp::path!("profile"))
        .and(public_user_filter(context.clone()))
        .and(context_filter.clone())
        .map(get);

    // POST /profile {email?} -> 200
    let set_profile = warp::post()
        .and(warp::path!("profile"))
        .and(public_user_filter(context.clone()))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .map(update);

    // DELETE /profile -> 200
    let delete_profile = warp::delete()
        .and(warp::path!("profile"))
        .and(public_user_filter(context.clone()))
        .and(context_filter.clone())
        .map(delete);

    get_profile.or(set_profile).or(delete_profile).boxed()
}

fn get(user: PublicUser, context: Context) -> impl Reply {
    warp::reply::json(&Profile {
        id: user.id,
        email: None,
        organization: Some(Organization {
            id: user.id,
            name: String::from("Creatiwity"),
        }),
    })
}

fn update(user: PublicUser, data: ProfileForm, context: Context) -> impl Reply {
    warp::reply()
}

fn delete(user: PublicUser, context: Context) -> impl Reply {
    warp::reply()
}

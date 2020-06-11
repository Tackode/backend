use super::super::authorization::public_user_filter;
use super::super::common::*;
use super::super::types::*;
use crate::model::user;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

pub fn routes(context: Context) -> BoxedFilter<(impl Reply,)> {
    let moved_context = context.clone();
    let context_filter = warp::any().map(move || moved_context.clone());

    // GET /profile -> Profile(Organization)
    let get_profile = warp::get()
        .and(warp::path!("profile"))
        .and(public_user_filter(context.clone()))
        .and(context_filter.clone())
        .and_then(get);

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

async fn get(public: PublicUser, context: Context) -> Result<impl Reply, Rejection> {
    let connectors = context.builders.create();

    let profile: Profile = user::get_with_organization(&connectors, &public.user.id)?.into();

    Ok(warp::reply::json(&profile))
}

fn update(public: PublicUser, data: ProfileForm, context: Context) -> impl Reply {
    warp::reply()
}

fn delete(public: PublicUser, context: Context) -> impl Reply {
    warp::reply()
}

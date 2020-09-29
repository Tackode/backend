use super::super::authorization::public_user_filter;
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

    // DELETE /profile -> 200
    let delete_profile = warp::delete()
        .and(warp::path!("profile"))
        .and(public_user_filter(context.clone()))
        .and(context_filter.clone())
        .and_then(delete);

    get_profile.or(delete_profile).boxed()
}

async fn get(public: PublicUser, context: Context) -> Result<impl Reply, Rejection> {
    let connector = context.builders.create();

    let profile: Profile = user::get_with_organization(&connector, &public.user.id)?.into();

    Ok(warp::reply::json(&profile))
}

async fn delete(public: PublicUser, context: Context) -> Result<impl Reply, Rejection> {
    user::delete(&context.builders.create(), &public.user.id)?;

    Ok(warp::reply())
}

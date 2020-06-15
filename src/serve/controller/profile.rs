use super::super::authorization::public_user_filter;
use super::super::error::Error;
use super::super::types::*;
use crate::model::user;
use validator::Validate;
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

    // PUT /profile {email?} -> 200
    let set_profile = warp::put()
        .and(warp::path!("profile"))
        .and(public_user_filter(context.clone()))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .and_then(update);

    // DELETE /profile -> 200
    let delete_profile = warp::delete()
        .and(warp::path!("profile"))
        .and(public_user_filter(context.clone()))
        .and(context_filter.clone())
        .and_then(delete);

    get_profile.or(set_profile).or(delete_profile).boxed()
}

async fn get(public: PublicUser, context: Context) -> Result<impl Reply, Rejection> {
    let connectors = context.builders.create();

    let profile: Profile = user::get_with_organization(&connectors, &public.user.id)?.into();

    Ok(warp::reply::json(&profile))
}

async fn update(
    public: PublicUser,
    data: ProfileForm,
    context: Context,
) -> Result<impl Reply, Rejection> {
    // Validate data
    if let Err(errors) = data.validate() {
        return Err(warp::reject::custom(Error::InvalidDataWithDetails {
            source: errors,
        }));
    }

    user::set_email(&context.builders.create(), &public.user.id, &data.email)?;

    Ok(warp::reply())
}

async fn delete(public: PublicUser, context: Context) -> Result<impl Reply, Rejection> {
    user::delete(&context.builders.create(), &public.user.id)?;

    Ok(warp::reply())
}

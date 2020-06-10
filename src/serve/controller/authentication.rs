use super::super::authorization::public_user_filter;
use super::super::common::*;
use uuid::Uuid;
use warp::{filters::BoxedFilter, Filter, Reply};

pub fn routes(context: Context) -> BoxedFilter<(impl Reply,)> {
    let moved_context = context.clone();
    let context_filter = warp::any().map(move || moved_context.clone());

    // POST /login {email, role, organization_name?} -> 200
    let login = warp::post()
        .and(warp::path!("login"))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .map(login);

    // POST /logout -> 200
    let logout = warp::post()
        .and(warp::path!("logout"))
        .and(public_user_filter(context.clone()))
        .and(context_filter.clone())
        .map(logout);

    // POST /session/<session_id>/validate {confirmation_token} -> Credentials
    let session_validate = warp::post()
        .and(warp::path!("session" / Uuid / "validate"))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .map(validate);

    login.or(logout).or(session_validate).boxed()
}

fn validate(session_id: Uuid, data: ValidateSessionForm, context: Context) -> impl Reply {
    warp::reply::json(&Credentials {
        login: String::from("LOGIN"),
        token: String::from("TOKEN"),
    })
}

fn login(data: LoginForm, context: Context) -> impl Reply {
    // Rate limit if more than 3 unconfirmed in the last 4 minutes
    warp::reply::json(&Session {
        session_id: Uuid::parse_str("85f520d0-193d-4386-bdf6-902bc7a4350e").unwrap(),
    })
}

fn logout(user: PublicUser, context: Context) -> impl Reply {
    // Remove token from session
    warp::reply()
}

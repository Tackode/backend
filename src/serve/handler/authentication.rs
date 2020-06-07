use super::super::common::*;
use uuid::Uuid;
use warp::reply::Reply;

pub fn validate(session_id: Uuid, data: ValidateSessionForm, context: Context) -> impl Reply {
    warp::reply::json(&Credentials {
        login: String::from("LOGIN"),
        token: String::from("TOKEN"),
    })
}

pub fn login(data: LoginForm, context: Context) -> impl Reply {
    // Rate limit if more than 3 unconfirmed in the last 4 minutes
    warp::reply::json(&Session {
        session_id: Uuid::parse_str("85f520d0-193d-4386-bdf6-902bc7a4350e").unwrap(),
    })
}

pub fn logout(user: PublicUser, context: Context) -> impl Reply {
    // Remove token from session
    warp::reply()
}

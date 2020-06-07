use super::super::common::*;
use warp::reply::Reply;

pub fn validate(device_id: String, data: ValidateDeviceForm, context: Context) -> impl Reply {
    warp::reply::json(&Credentials {
        login: String::from("LOGIN"),
        token: String::from("TOKEN"),
    })
}

pub fn login(data: LoginForm, context: Context) -> impl Reply {
    // Rate limit if more than 3 unconfirmed in the last 4 minutes
    warp::reply()
}

pub fn logout(user: PublicUser, context: Context) -> impl Reply {
    // Remove token from device
    warp::reply()
}

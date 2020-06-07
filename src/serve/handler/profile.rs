use super::super::common::*;
use warp::reply::Reply;

pub fn get(user: PublicUser, context: Context) -> impl Reply {
    warp::reply::json(&Profile {
        id: user.id,
        email: None,
        organization: Some(Organization {
            id: user.id,
            name: String::from("Creatiwity"),
        }),
    })
}

pub fn update(user: PublicUser, data: ProfileForm, context: Context) -> impl Reply {
    warp::reply()
}

pub fn delete(user: PublicUser, context: Context) -> impl Reply {
    warp::reply()
}

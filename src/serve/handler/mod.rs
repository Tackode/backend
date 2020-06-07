pub mod authentication;
pub mod checkin;
pub mod infection;
pub mod organization;
pub mod place;
pub mod profile;

use super::common::*;
use warp::reply::Reply;

pub fn index() -> impl Reply {
    warp::reply::json(&HealthResponse { healthy: true })
}

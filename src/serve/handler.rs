use super::common::*;
use warp::reply::Reply;

pub fn index() -> impl Reply {
    warp::reply::json(&HealthResponse { healthy: true })
}

pub fn scan(query: ScanQuery, context: Context) -> impl Reply {
    warp::reply::with_status(format!("{}", query.uuid), warp::http::StatusCode::OK)
}

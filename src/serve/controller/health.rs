use super::super::common::*;
use warp::{filters::BoxedFilter, Filter, Reply};

pub fn routes() -> BoxedFilter<(impl Reply,)> {
    // GET / -> HealthResponse
    warp::get().and(warp::path::end()).map(index).boxed()
}

fn index() -> impl Reply {
    warp::reply::json(&HealthResponse { healthy: true })
}

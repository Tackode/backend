use super::error::Error;
use serde::de::DeserializeOwned;
use warp::Filter;

pub fn query_qs<T: DeserializeOwned>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Copy
{
    warp::query::raw().and_then(|query_string: String| async move {
        serde_qs::from_str::<T>(&query_string).map_err(|e| {
            tracing::debug!("failed to decode query string '{}': {:?}", query_string, e);
            warp::reject::custom(Error::InvalidData)
        })
    })
}

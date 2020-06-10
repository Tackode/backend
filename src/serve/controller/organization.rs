use super::super::authorization::professional_user_filter;
use super::super::common::*;
use warp::{filters::BoxedFilter, Filter, Reply};

pub fn routes(context: Context) -> BoxedFilter<(impl Reply,)> {
    let moved_context = context.clone();
    let context_filter = warp::any().map(move || moved_context.clone());

    // POST /organization {name} -> 200
    warp::post()
        .and(warp::path!("organization"))
        .and(professional_user_filter(context.clone()))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .map(update)
        .boxed()
}

fn update(user: ProfessionalUser, data: OrganizationForm, context: Context) -> impl Reply {
    warp::reply()
}

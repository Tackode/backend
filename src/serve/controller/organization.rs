use super::super::authorization::professional_user_filter;
use super::super::error::Error;
use super::super::types::*;
use crate::model::organization;
use validator::Validate;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

pub fn routes(context: Context) -> BoxedFilter<(impl Reply,)> {
    let moved_context = context.clone();
    let context_filter = warp::any().map(move || moved_context.clone());

    // PUT /organization {name} -> 200
    warp::put()
        .and(warp::path!("organization"))
        .and(professional_user_filter(context.clone()))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .and_then(update)
        .boxed()
}

async fn update(
    professional: ProfessionalUser,
    data: OrganizationForm,
    context: Context,
) -> Result<impl Reply, Rejection> {
    // Validate data
    if let Err(errors) = data.validate() {
        return Err(warp::reject::custom(Error::InvalidDataWithDetails {
            source: errors,
        }));
    }

    let connector = context.builders.create();

    organization::set_name(&connector, &professional.organization.id, &data.name)?;

    Ok(warp::reply())
}

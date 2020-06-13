use super::super::authorization::professional_user_filter;
use super::super::error::Error;
use super::super::types::*;
use crate::model::{infection, place};
use chrono::{Duration, Utc};
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

pub fn routes(context: Context) -> BoxedFilter<(impl Reply,)> {
    let moved_context = context.clone();
    let context_filter = warp::any().map(move || moved_context.clone());

    // POST /infection -> 200
    let create_infection = warp::post()
        .and(warp::path!("infection"))
        .and(professional_user_filter(context.clone()))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .and_then(create);

    // GET /infections -> Vec<Infection>
    let get_infections = warp::get()
        .and(warp::path!("infections"))
        .and(professional_user_filter(context.clone()))
        .and(context_filter.clone())
        .map(get_all);

    create_infection.or(get_infections).boxed()
}

async fn create(
    professional: ProfessionalUser,
    data: InfectionForm,
    context: Context,
) -> Result<impl Reply, Rejection> {
    // Validate dates and places
    if data.start_timestamp >= data.end_timestamp
        || data.end_timestamp > Utc::now()
        || data.end_timestamp - data.start_timestamp >= Duration::minutes(480)
    {
        return Err(warp::reject::custom(Error::InvalidData));
    }

    let connectors = context.builders.create();

    place::validate_places_owned(&connectors, &professional.organization.id, &data.places_ids)?;

    // Insert infection
    let infection: Infection = infection::insert(
        &connectors,
        &infection::InfectionInsert {
            organization_id: professional.organization.id,
            places_ids: data.places_ids,
            start_timestamp: data.start_timestamp,
            end_timestamp: data.end_timestamp,
        },
    )?
    .into();

    Ok(warp::reply::json(&infection))
}

fn get_all(professional: ProfessionalUser, context: Context) -> impl Reply {
    warp::reply()
}

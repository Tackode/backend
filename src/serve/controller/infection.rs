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
        .and_then(get_all);

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
    let infection_id = infection::insert(
        &connectors,
        &infection::InfectionInsert {
            organization_id: professional.organization.id,
            places_ids: data.places_ids,
            start_timestamp: data.start_timestamp,
            end_timestamp: data.end_timestamp,
        },
    )?;

    let new_infection: Infection =
        infection::get_with_organization(&connectors, &infection_id)?.into();

    Ok(warp::reply::json(&new_infection))
}

async fn get_all(
    professional: ProfessionalUser,
    context: Context,
) -> Result<impl Reply, Rejection> {
    let connectors = context.builders.create();

    let infections: Vec<Infection> =
        infection::get_all_with_organization(&connectors, &professional.organization.id)?
            .into_iter()
            .map(|i| i.into())
            .collect();

    Ok(warp::reply::json(&infections))
}

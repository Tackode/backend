use super::super::authorization::professional_user_filter;
use super::super::error::Error;
use super::super::types::*;
use crate::connector::email::template::InfectionWarningEmail;
use crate::model::{checkin, infection, place};
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
        || data.end_timestamp - data.start_timestamp >= Duration::minutes(720)
    {
        return Err(warp::reject::custom(Error::InvalidData));
    }

    let connector = context.builder.create();

    place::validate_places_owned(&connector, &professional.organization.id, &data.places_ids)?;

    // Insert infection
    let infection_id = infection::insert(
        &connector,
        &infection::InfectionInsert {
            organization_id: professional.organization.id,
            places_ids: data.places_ids.clone(),
            start_timestamp: data.start_timestamp,
            end_timestamp: data.end_timestamp,
        },
    )?;

    // Notify infected
    let infected_users = checkin::get_potential_infections(
        &connector,
        &data.places_ids,
        &data.start_timestamp,
        &data.end_timestamp,
    )?;

    connector
        .email
        .send(
            infected_users
                .iter()
                .map(|(checkin, user, place)| InfectionWarningEmail {
                    to: user.email.clone(),
                    organization_name: professional.organization.name.clone(),
                    place_name: place.name.clone(),
                    checkin_datetime: checkin.start_timestamp,
                })
                .collect(),
        )
        .await;

    let new_infection: Infection =
        infection::get_with_organization(&connector, &infection_id)?.into();

    Ok(warp::reply::json(&new_infection))
}

async fn get_all(
    professional: ProfessionalUser,
    context: Context,
) -> Result<impl Reply, Rejection> {
    let connector = context.builder.create();

    let infections: Vec<Infection> =
        infection::get_all_with_organization(&connector, &professional.organization.id)?
            .into_iter()
            .map(|i| i.into())
            .collect();

    Ok(warp::reply::json(&infections))
}

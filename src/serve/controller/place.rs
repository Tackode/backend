use super::super::authorization::professional_user_filter;
use super::super::common::*;
use super::super::error::Error;
use super::super::types::*;
use crate::model::place;
use uuid::Uuid;
use validator::Validate;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

pub fn routes(context: Context) -> BoxedFilter<(impl Reply,)> {
    let moved_context = context.clone();
    let context_filter = warp::any().map(move || moved_context.clone());

    // GET /place/<id> -> Place
    let get_place = warp::get()
        .and(warp::path!("place" / Uuid))
        .and(context_filter.clone())
        .map(get_one);

    // GET /places -> Vec<Place>
    let get_places = warp::get()
        .and(warp::path!("places"))
        .and(professional_user_filter(context.clone()))
        .and(context_filter.clone())
        .and_then(get_all);

    // POST /place/<id> -> Place
    let create_place = warp::post()
        .and(warp::path!("place"))
        .and(professional_user_filter(context.clone()))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .and_then(create);

    // POST /place/<id> -> Place
    let set_place = warp::post()
        .and(warp::path!("place" / Uuid))
        .and(professional_user_filter(context.clone()))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .and_then(update);

    // DELETE /place/<id> -> 200
    let delete_place = warp::delete()
        .and(warp::path!("place" / Uuid))
        .and(professional_user_filter(context.clone()))
        .and(context_filter.clone())
        .map(delete);

    get_place
        .or(get_places)
        .or(create_place)
        .or(set_place)
        .or(delete_place)
        .boxed()
}

fn get_one(place_id: Uuid, context: Context) -> impl Reply {
    warp::reply::json(&Place {
        id: place_id,
        organization: Organization {
            id: place_id,
            name: String::from("Creatiwity"),
        },
        name: String::from("Bureau"),
        description: None,
        average_duration: 480,
    })
}

async fn get_all(
    professional: ProfessionalUser,
    context: Context,
) -> Result<impl Reply, Rejection> {
    let place_id = Uuid::parse_str("85f520d0-193d-4386-bdf6-902bc7a4350e").unwrap();

    let connectors = context.builders.create();

    let places: Vec<Place> =
        place::get_all_with_organization(&connectors, &professional.organization.id)?
            .into_iter()
            .map(|p| p.into())
            .collect();

    Ok(warp::reply::json(&places))
}

async fn create(
    professional: ProfessionalUser,
    data: PlaceForm,
    context: Context,
) -> Result<impl Reply, Rejection> {
    // Validate data
    if let Err(errors) = data.validate() {
        return Err(warp::reject::custom(Error::InvalidDataWithDetails {
            source: errors,
        }));
    }

    let connectors = context.builders.create();

    // Create place
    let place_id = place::insert(
        &connectors,
        &place::PlaceInsert {
            organization_id: professional.organization.id,
            name: data.name,
            description: data.description,
            average_duration: data.average_duration,
        },
    )?;

    // Retrieve newly created place
    let place: Place = place::get_with_organization(&connectors, &place_id)?.into();

    Ok(warp::reply::json(&place))
}

async fn update(
    place_id: Uuid,
    professional: ProfessionalUser,
    data: PlaceForm,
    context: Context,
) -> Result<impl Reply, Rejection> {
    // Validate data
    if let Err(errors) = data.validate() {
        return Err(warp::reject::custom(Error::InvalidDataWithDetails {
            source: errors,
        }));
    }

    let connectors = context.builders.create();

    // Update place
    place::update(
        &connectors,
        &place_id,
        &professional.organization.id,
        &place::PlaceUpdate {
            name: data.name,
            description: data.description,
            average_duration: data.average_duration,
        },
    )?;

    Ok(warp::reply())
}

fn delete(place_id: Uuid, professional: ProfessionalUser, context: Context) -> impl Reply {
    warp::reply()
}

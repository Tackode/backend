use super::super::authorization::professional_user_filter;
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
        .and_then(get_one);

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

    // PUT /place/<id> -> Place
    let set_place = warp::put()
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
        .and_then(delete);

    get_place
        .or(get_places)
        .or(create_place)
        .or(set_place)
        .or(delete_place)
        .boxed()
}

async fn get_one(place_id: Uuid, context: Context) -> Result<impl Reply, Rejection> {
    let connector = context.builder.create();

    let place: Place = place::get_with_organization(&connector, &place_id)?.into();

    Ok(warp::reply::json(&place))
}

async fn get_all(
    professional: ProfessionalUser,
    context: Context,
) -> Result<impl Reply, Rejection> {
    let connector = context.builder.create();

    let places: Vec<Place> =
        place::get_all_with_organization(&connector, &professional.organization.id)?
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

    let connector = context.builder.create();

    // Create place
    let place_id = place::insert(
        &connector,
        &place::PlaceInsert {
            organization_id: professional.organization.id,
            name: data.name,
            description: data.description,
            average_duration: data.average_duration,
            maximum_gauge: data.maximum_gauge,
            address: data.address,
            latitude: data.latitude,
            longitude: data.longitude,
            maximum_duration: data.maximum_duration,
        },
    )?;

    // Retrieve newly created place
    let place: Place = place::get_with_organization(&connector, &place_id)?.into();

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

    let connector = context.builder.create();

    // Update place
    place::update(
        &connector,
        &place_id,
        &professional.organization.id,
        &place::PlaceUpdate {
            name: data.name,
            description: data.description,
            average_duration: data.average_duration,
            maximum_gauge: data.maximum_gauge,
            address: data.address,
            latitude: data.latitude,
            longitude: data.longitude,
            maximum_duration: data.maximum_duration,
        },
    )?;

    Ok(warp::reply())
}

async fn delete(
    place_id: Uuid,
    professional: ProfessionalUser,
    context: Context,
) -> Result<impl Reply, Rejection> {
    let connector = context.builder.create();

    // Update place
    place::set_disabled(&connector, &place_id, &professional.organization.id, true)?;

    Ok(warp::reply())
}

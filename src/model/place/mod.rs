mod common;

use super::error::{is_one, Error};
use super::organization::Organization;
use super::schema::{organization, place::dsl};
use super::types::*;
use crate::connector::Connector;
use crate::types::{Pagination, PaginationQuery};
use diesel::prelude::*;
use diesel::sql_types::*;
use postgis::ewkb::Point;
use postgis_diesel::*;
use uuid::Uuid;

pub use common::*;

pub fn get(connector: &Connector, id: &Uuid) -> Result<Place, Error> {
    let connection = connector.local.pool.get()?;

    dsl::place
        .filter(dsl::id.eq(id).and(dsl::disabled.eq(false)))
        .first::<Place>(&connection)
        .map_err(|error| error.into())
}

pub fn refresh_all_gauges(connector: &Connector) -> Result<usize, Error> {
    let connection = connector.local.pool.get()?;

    connection
        .transaction::<usize, Error, _>(|| {
            let updated = diesel::sql_query(
                "UPDATE place
                SET current_gauge = checkin.active_count
                FROM (SELECT place_id, SUM(number) as active_count
                    FROM checkin
                    WHERE start_timestamp <= NOW() AND end_timestamp >= NOW()
                    GROUP BY place_id) as checkin
                WHERE checkin.place_id = place.id AND disabled = FALSE",
            )
            .execute(&connection)?;

            diesel::sql_query(
                "UPDATE place
                SET
                    current_gauge_percent = NULL,
                    current_gauge_level = 'unknown'
                WHERE maximum_gauge IS NULL AND disabled = FALSE",
            )
            .execute(&connection)?;

            diesel::sql_query(
                "UPDATE place
                SET
                    current_gauge_percent = (current_gauge * 100) / maximum_gauge ,
                    current_gauge_level = 'safe'
                WHERE maximum_gauge IS NOT NULL AND disabled = FALSE",
            )
            .execute(&connection)?;

            diesel::sql_query(
                "UPDATE place
                SET current_gauge_level = 'warning'
                WHERE current_gauge_percent >= $1 AND disabled = FALSE",
            )
            .bind::<BigInt, _>(connector.configuration.gauge.warning)
            .execute(&connection)?;

            diesel::sql_query(
                "UPDATE place
                SET current_gauge_level = 'alert'
                WHERE current_gauge_percent >= $1 AND disabled = FALSE",
            )
            .bind::<BigInt, _>(connector.configuration.gauge.alert)
            .execute(&connection)?;

            Ok(updated)
        })
        .map_err(|error| error.into())
}

pub fn get_with_organization(
    connector: &Connector,
    id: &Uuid,
) -> Result<(Place, Organization), Error> {
    let connection = connector.local.pool.get()?;

    dsl::place
        .inner_join(organization::dsl::organization)
        .filter(dsl::id.eq(id).and(dsl::disabled.eq(false)))
        .first::<(Place, Organization)>(&connection)
        .map_err(|error| error.into())
}

pub fn get_all_with_organization(
    connector: &Connector,
    organization_id: &Uuid,
) -> Result<Vec<(Place, Organization)>, Error> {
    let connection = connector.local.pool.get()?;

    dsl::place
        .inner_join(organization::dsl::organization)
        .filter(
            organization::dsl::id
                .eq(organization_id)
                .and(dsl::disabled.eq(false)),
        )
        .order(dsl::created_at.desc())
        .load::<(Place, Organization)>(&connection)
        .map_err(|error| error.into())
}

pub fn search(
    connector: &Connector,
    location: PointC<Point>,
    radius_in_meters: i64,
    gauge_levels: Vec<GaugeLevel>,
    pagination: PaginationQuery,
) -> Result<(Pagination, Vec<PlaceSearchResult>), Error> {
    let connection = connector.local.pool.get()?;

    let mut places = diesel::sql_query(format!(
        "
        WITH c AS (
            SELECT ST_SetSRID(ST_MakePoint($1, $2), 4326)::geography AS center
        )
        SELECT place.id,
            place.organization_id,
            place.name,
            place.description,
            place.average_duration,
            place.disabled,
            place.created_at,
            place.updated_at,
            place.maximum_gauge,
            place.address,
            place.maximum_duration,
            place.current_gauge,
            place.location,
            place.current_gauge_level,
            place.current_gauge_percent,
            organization.id AS org_id,
            organization.user_id AS org_user_id,
            organization.name AS org_name,
            organization.confirmed AS org_confirmed,
            organization.disabled AS org_disabled,
            organization.updated_at AS org_updated_at,
            organization.created_at AS org_created_at,
            ST_Distance(place.location, c.center, false) AS meter_distance
        FROM place
        JOIN c ON TRUE
        INNER JOIN organization
        ON place.organization_id = organization.id
        WHERE
            place.disabled = FALSE
            AND current_gauge_level IN ({})
            AND ST_DWithin(
                place.location,
                center,
                $3,
                false
            )
        ORDER BY meter_distance ASC
        LIMIT $4 OFFSET $5",
        join_gauge_levels(gauge_levels)
    ))
    .bind::<Double, _>(location.v.x)
    .bind::<Double, _>(location.v.y)
    .bind::<BigInt, _>(radius_in_meters)
    .bind::<BigInt, _>(pagination.limit + 1)
    .bind::<BigInt, _>((pagination.page - 1) * pagination.limit)
    .load::<PlaceSearchRow>(&connection)?;

    let next_page = if places.len() as i64 == pagination.limit + 1 {
        places.remove(places.len() - 1);
        Some(pagination.page + 1)
    } else {
        None
    };

    Ok((
        Pagination {
            page: pagination.page,
            next_page,
        },
        places.into_iter().map(|place| place.into()).collect(),
    ))
}

pub fn validate_places_owned(
    connector: &Connector,
    organization_id: &Uuid,
    places_ids: &Vec<Uuid>,
) -> Result<(), Error> {
    let connection = connector.local.pool.get()?;
    let length = places_ids.len() as i64;

    dsl::place
        .select(diesel::dsl::count(dsl::id))
        .filter(
            dsl::organization_id
                .eq(organization_id)
                .and(dsl::id.eq_any(places_ids)),
        )
        .first(&connection)
        .map_err(|error| error.into())
        .and_then(|count: i64| {
            if count == length {
                Ok(())
            } else {
                Err(Error::NotFoundWithName {
                    name: String::from("Place"),
                })
            }
        })
}

pub fn insert(connector: &Connector, place: &PlaceInsert) -> Result<Uuid, Error> {
    let connection = connector.local.pool.get()?;

    diesel::insert_into(dsl::place)
        .values(place)
        .returning(dsl::id)
        .get_result(&connection)
        .map_err(|error| error.into())
}

pub fn update(
    connector: &Connector,
    id: &Uuid,
    organization_id: &Uuid,
    place: &PlaceUpdate,
) -> Result<(), Error> {
    let connection = connector.local.pool.get()?;

    diesel::update(
        dsl::place.filter(
            dsl::id
                .eq(id)
                .and(dsl::organization_id.eq(organization_id))
                .and(dsl::disabled.eq(false)),
        ),
    )
    .set(place)
    .execute(&connection)
    .map_err(|error| error.into())
    .and_then(|count| is_one(count, "Place"))
}

pub fn set_disabled(
    connector: &Connector,
    id: &Uuid,
    organization_id: &Uuid,
    disabled: bool,
) -> Result<(), Error> {
    let connection = connector.local.pool.get()?;

    diesel::update(
        dsl::place.filter(
            dsl::id
                .eq(id)
                .and(dsl::organization_id.eq(organization_id))
                .and(dsl::disabled.eq(false)),
        ),
    )
    .set(dsl::disabled.eq(disabled))
    .execute(&connection)
    .map_err(|error| error.into())
    .and_then(|count| is_one(count, "Place"))
}

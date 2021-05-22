#![allow(unused_imports)]

table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;
    use crate::model::types::*;

    checkin (id) {
        id -> Uuid,
        place_id -> Uuid,
        session_id -> Uuid,
        user_id -> Uuid,
        start_timestamp -> Timestamptz,
        end_timestamp -> Timestamptz,
        duration -> Int8,
        potential_infection -> Bool,
        confirmed -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        number -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;
    use crate::model::types::*;

    infection (id) {
        id -> Uuid,
        organization_id -> Uuid,
        places_ids -> Array<Uuid>,
        start_timestamp -> Timestamptz,
        end_timestamp -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;
    use crate::model::types::*;

    opening_hour_computed (id) {
        id -> Uuid,
        place_id -> Uuid,
        opening_timestamp -> Timestamptz,
        closing_timestamp -> Timestamptz,
        evacuation_timestamp -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;
    use crate::model::types::*;

    opening_hour_date (id) {
        id -> Uuid,
        place_id -> Uuid,
        date -> Date,
        opening_time -> Nullable<Time>,
        closure_time -> Nullable<Time>,
        evacuation_time -> Nullable<Time>,
        closed -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;
    use crate::model::types::*;

    opening_hour_day (id) {
        id -> Uuid,
        place_id -> Uuid,
        day -> Int2,
        opening_time -> Time,
        closure_time -> Time,
        evacuation_time -> Time,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;
    use crate::model::types::*;

    organization (id) {
        id -> Uuid,
        user_id -> Uuid,
        name -> Text,
        confirmed -> Bool,
        disabled -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;
    use crate::model::types::*;

    place (id) {
        id -> Uuid,
        organization_id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        average_duration -> Int8,
        disabled -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        maximum_gauge -> Nullable<Int8>,
        address -> Nullable<Text>,
        maximum_duration -> Int8,
        current_gauge -> Int8,
        location -> Nullable<Geometry>,
        current_gauge_level -> Gauge_level,
        current_gauge_percent -> Nullable<Int8>,
        timezone -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;
    use crate::model::types::*;

    session (id) {
        id -> Uuid,
        user_id -> Uuid,
        description -> Text,
        hashed_token -> Nullable<Text>,
        hashed_confirmation_token -> Nullable<Text>,
        confirmed -> Bool,
        disabled -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;
    use crate::model::types::*;

    user (id) {
        id -> Uuid,
        login -> Text,
        email -> Text,
        role -> Text,
        confirmed -> Bool,
        disabled -> Bool,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

joinable!(checkin -> place (place_id));
joinable!(checkin -> session (session_id));
joinable!(checkin -> user (user_id));
joinable!(infection -> organization (organization_id));
joinable!(opening_hour_computed -> place (place_id));
joinable!(opening_hour_date -> place (place_id));
joinable!(opening_hour_day -> place (place_id));
joinable!(organization -> user (user_id));
joinable!(place -> organization (organization_id));
joinable!(session -> user (user_id));

allow_tables_to_appear_in_same_query!(
    checkin,
    infection,
    opening_hour_computed,
    opening_hour_date,
    opening_hour_day,
    organization,
    place,
    session,
    user,
);

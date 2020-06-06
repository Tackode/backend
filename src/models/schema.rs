table! {
    checkin (id) {
        id -> Uuid,
        place_id -> Uuid,
        device_id -> Uuid,
        user_id -> Uuid,
        start_timestamp -> Timestamptz,
        end_timestamp -> Timestamptz,
        duration -> Int8,
        potential_infection -> Bool,
        confirmed -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    device (id) {
        id -> Uuid,
        user_id -> Uuid,
        description -> Text,
        hashed_token -> Text,
        hashed_confirmation_token -> Nullable<Text>,
        confirmed -> Bool,
        disabled -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
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
    place (id) {
        id -> Uuid,
        organization_id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        average_duration -> Int8,
        disabled -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    user (id) {
        id -> Uuid,
        login -> Text,
        role -> Text,
        confirmed -> Bool,
        disabled -> Bool,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

joinable!(checkin -> device (device_id));
joinable!(checkin -> place (place_id));
joinable!(checkin -> user (user_id));
joinable!(device -> user (user_id));
joinable!(infection -> organization (organization_id));
joinable!(organization -> user (user_id));
joinable!(place -> organization (organization_id));

allow_tables_to_appear_in_same_query!(
    checkin,
    device,
    infection,
    organization,
    place,
    user,
);

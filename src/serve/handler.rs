use super::common::*;
use uuid::Uuid;
use warp::reply::Reply;

pub fn index() -> impl Reply {
    warp::reply::json(&HealthResponse { healthy: true })
}

pub fn get_place(place_id: Uuid, context: Context) -> impl Reply {
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

pub fn get_places(user: ProfessionalUser, context: Context) -> impl Reply {
    let place_id = Uuid::parse_str("85f520d0-193d-4386-bdf6-902bc7a4350e").unwrap();

    warp::reply::json(&vec![Place {
        id: place_id,
        organization: Organization {
            id: place_id,
            name: String::from("Creatiwity"),
        },
        name: String::from("Bureau"),
        description: None,
        average_duration: 480,
    }])
}

pub fn create_place(user: ProfessionalUser, data: PlaceForm, context: Context) -> impl Reply {
    warp::reply()
}

pub fn set_place(
    place_id: Uuid,
    user: ProfessionalUser,
    data: PlaceForm,
    context: Context,
) -> impl Reply {
    warp::reply()
}

pub fn delete_place(place_id: Uuid, user: ProfessionalUser, context: Context) -> impl Reply {
    warp::reply()
}

pub fn checkin(data: CheckinForm, context: Context) -> impl Reply {
    warp::reply()
}

pub fn device_validate(
    device_id: String,
    data: ValidateDeviceForm,
    context: Context,
) -> impl Reply {
    warp::reply::json(&Credentials {
        login: String::from("LOGIN"),
        token: String::from("TOKEN"),
    })
}

pub fn get_profile(user: PublicUser, context: Context) -> impl Reply {
    warp::reply::json(&Profile {
        id: user.id,
        email: None,
        organization: Some(Organization {
            id: user.id,
            name: String::from("Creatiwity"),
        }),
    })
}

pub fn set_profile(user: PublicUser, data: ProfileForm, context: Context) -> impl Reply {
    warp::reply()
}

pub fn delete_profile(user: PublicUser, context: Context) -> impl Reply {
    warp::reply()
}

pub fn set_organization(
    user: ProfessionalUser,
    data: OrganizationForm,
    context: Context,
) -> impl Reply {
    warp::reply()
}

pub fn checkins(user: PublicUser, context: Context) -> impl Reply {
    warp::reply::json(&vec![Checkin {
        id: user.id,
        timestamp: chrono::Utc::now(),
        duration: 60,
        place: Place {
            id: user.id,
            organization: Organization {
                id: user.id,
                name: String::from("Creatiwity"),
            },
            name: String::from("Bureau 1"),
            description: None,
            average_duration: 600,
        },
    }])
}

pub fn login(data: LoginForm, context: Context) -> impl Reply {
    // Rate limit if more than 3 unconfirmed in the last 4 minutes
    warp::reply()
}

pub fn logout(user: PublicUser, context: Context) -> impl Reply {
    // Remove token from device
    warp::reply()
}

pub fn create_infection(
    user: ProfessionalUser,
    data: InfectionForm,
    context: Context,
) -> impl Reply {
    warp::reply()
}

pub fn get_infections(user: ProfessionalUser, context: Context) -> impl Reply {
    let placeholder_id = Uuid::parse_str("85f520d0-193d-4386-bdf6-902bc7a4350e").unwrap();

    warp::reply::json(&vec![Infection {
        id: placeholder_id,
        start_timestamp: chrono::Utc::now(),
        end_timestamp: chrono::Utc::now(),
        places: vec![Place {
            id: user.id,
            organization: Organization {
                id: user.id,
                name: String::from("Creatiwity"),
            },
            name: String::from("Bureau 1"),
            description: None,
            average_duration: 600,
        }],
    }])
}

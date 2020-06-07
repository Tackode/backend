use super::common::*;
use warp::reply::Reply;

pub fn index() -> impl Reply {
    warp::reply::json(&HealthResponse { healthy: true })
}

pub fn scan(query: ScanQuery, context: Context) -> impl Reply {
    warp::reply::json(&Place {
        id: query.place_id,
        organization: Organization {
            id: query.place_id,
            name: String::from("Creatiwity"),
        },
        name: String::from("Bureau"),
        description: None,
        average_duration: 480,
    })
}

pub fn checkin(data: CheckinForm, context: Context) -> impl Reply {
    warp::reply()
}

pub fn validate_device(data: ValidateDeviceForm, context: Context) -> impl Reply {
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

pub fn set_profile(user: PublicUser, profile: ProfileForm, context: Context) -> impl Reply {
    warp::reply()
}

pub fn set_organization(
    user: ProfessionalUser,
    organization: OrganizationForm,
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

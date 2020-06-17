use super::super::authorization::public_user_filter;
use super::super::error::Error;
use super::super::session::{create_session, get_auth_from_email};
use super::super::types::*;
use crate::model::{checkin, place, user};
use chrono::{Duration, Utc};
use validator::Validate;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

pub fn routes(context: Context) -> BoxedFilter<(impl Reply,)> {
    let moved_context = context.clone();
    let context_filter = warp::any().map(move || moved_context.clone());

    // POST /checkin {uuid, email, store_email, duration} -> 200
    let checkin = warp::post()
        .and(warp::path!("checkin"))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(warp::header::<String>("user-agent"))
        .and(
            public_user_filter(context.clone())
                .map(|u| Some(u))
                .or(warp::any().map(|| None))
                .unify(),
        )
        .and(context_filter.clone())
        .and_then(create);

    // GET /checkins -> Checkin(Place)
    let get_checkins = warp::get()
        .and(warp::path!("checkins"))
        .and(public_user_filter(context.clone()))
        .and(context_filter.clone())
        .and_then(get_all);

    checkin.or(get_checkins).boxed()
}

async fn create(
    data: CheckinForm,
    user_agent: String,
    public: Option<PublicUser>,
    context: Context,
) -> Result<impl Reply, Rejection> {
    // Validate data
    if let Err(errors) = data.validate() {
        return Err(warp::reject::custom(Error::InvalidDataWithDetails {
            source: errors,
        }));
    }

    // Prepare connector
    let connectors = context.builders.create();

    // Check if place exists
    place::get(&connectors, &data.place_id)?;

    // Hash email to get login
    let (login, stored_email) = get_auth_from_email(data.email.clone(), data.store_email);

    // Generate user and session
    let (user, session) = match public {
        Some(public) => {
            user::set_email_with_login(&connectors, &login, &stored_email)?;
            (public.user, public.session)
        }
        None => {
            let user: User = user::insert(
                &connectors,
                &user::UserInsert {
                    login,
                    email: stored_email,
                    role: user::UserRole::Public,
                },
                true,
            )?
            .into();

            let session = create_session(&connectors, user.id, data.email, user_agent)?;

            (user, session)
        }
    };

    // Create checkin
    checkin::insert(
        &connectors,
        &checkin::CheckinInsert {
            place_id: data.place_id,
            session_id: session.id,
            user_id: user.id,
            start_timestamp: Utc::now(),
            end_timestamp: Utc::now() + Duration::minutes(data.duration),
            duration: data.duration,
            confirmed: session.confirmed,
        },
    )?;

    // Return session_id
    Ok(warp::reply::json(&session))
}

async fn get_all(public: PublicUser, context: Context) -> Result<impl Reply, Rejection> {
    let connectors = context.builders.create();

    let checkins: Vec<Checkin> = checkin::get_all_with_user(&connectors, &public.user.id)?
        .into_iter()
        .map(|c| c.into())
        .collect();

    Ok(warp::reply::json(&checkins))
}

// TODO: delete all for user

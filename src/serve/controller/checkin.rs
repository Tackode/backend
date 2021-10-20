use super::super::authorization::public_user_filter;
use super::super::error::Error;
use super::super::session::{create_session, get_auth_from_email};
use super::super::types::*;
use crate::model::{checkin, place, user};
use chrono::{Duration, Utc};
use uuid::Uuid;
use validator::Validate;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

pub fn routes(context: Context) -> BoxedFilter<(impl Reply,)> {
    let moved_context = context.clone();
    let context_filter = warp::any().map(move || moved_context.clone());

    // POST /checkin {uuid, email, duration} -> 200
    let checkin = warp::post()
        .and(warp::path!("checkin"))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(warp::header::<String>("user-agent"))
        .and(
            public_user_filter(context.clone())
                .map(Some)
                .or(warp::any().map(|| None))
                .unify(),
        )
        .and(context_filter.clone())
        .and_then(create);

    // POST /checkin/:uuid/leave -> 200
    let leave = warp::post()
        .and(warp::path!("checkin" / Uuid / "leave"))
        .and(public_user_filter(context.clone()))
        .and(context_filter.clone())
        .and_then(leave);

    // GET /checkins -> Checkin(Place)
    let get_checkins = warp::get()
        .and(warp::path!("checkins"))
        .and(public_user_filter(context.clone()))
        .and(context_filter.clone())
        .and_then(get_all);

    // DELETE /checkins -> 200
    let delete_checkins = warp::delete()
        .and(warp::path!("checkins"))
        .and(public_user_filter(context))
        .and(context_filter)
        .and_then(delete_all);

    checkin
        .or(leave)
        .or(get_checkins)
        .or(delete_checkins)
        .boxed()
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
    let connector = context.builder.create();

    // Check if place exists
    let place = place::get(&connector, &data.place_id)?;

    // Check if place is full
    if let Some(maximum_gauge) = place.maximum_gauge {
        if maximum_gauge < place.current_gauge + data.number {
            return Err(warp::reject::custom(Error::MaximumGaugeReached));
        }
    }

    // Hash email to get login
    let (login, cleaned_email) = get_auth_from_email(data.email.clone());

    // Generate user and session
    let (user, session) = match public {
        Some(public) => {
            user::set_email_with_login(&connector, &login, &cleaned_email)?;
            (public.user, public.session)
        }
        None => {
            let user: User = user::insert(
                &connector,
                &user::UserInsert {
                    login,
                    email: cleaned_email,
                    role: user::UserRole::Public,
                },
                true,
            )?
            .into();

            let session = create_session(
                &connector,
                user.id,
                data.email,
                user_agent,
                RedirectPage::CheckinConfirmation {
                    place_id: data.place_id,
                },
            )
            .await?;

            (user, session)
        }
    };

    // Create checkin
    checkin::insert(
        &connector,
        &checkin::CheckinInsert {
            place_id: data.place_id,
            session_id: session.id,
            user_id: user.id,
            start_timestamp: Utc::now(),
            end_timestamp: Utc::now() + Duration::minutes(data.duration),
            duration: data.duration,
            confirmed: session.confirmed,
            number: data.number,
        },
    )?;

    // Return session_id
    Ok(warp::reply::json(&session))
}

async fn leave(
    checkin_id: Uuid,
    public: PublicUser,
    context: Context,
) -> Result<impl Reply, Rejection> {
    // Prepare connector
    let connector = context.builder.create();

    // End checkin
    checkin::leave(&connector, &public.user.id, &checkin_id)?;

    // Get checkin
    let checkin: Checkin = checkin::get(&connector, &checkin_id)?.into();

    // Return checkin
    Ok(warp::reply::json(&checkin))
}

async fn get_all(public: PublicUser, context: Context) -> Result<impl Reply, Rejection> {
    let connector = context.builder.create();

    let checkins: Vec<Checkin> = checkin::get_all_with_user(&connector, &public.user.id)?
        .into_iter()
        .map(|c| c.into())
        .collect();

    Ok(warp::reply::json(&checkins))
}

async fn delete_all(public: PublicUser, context: Context) -> Result<impl Reply, Rejection> {
    let connector = context.builder.create();

    checkin::delete_all_with_user(&connector, &public.user.id)?;

    Ok(warp::reply())
}

use super::super::authorization::public_user_filter;
use super::super::error::Error;
use super::super::session::{create_session, get_auth_from_email};
use super::super::types::*;
use crate::model::{checkin, organization, session, user};
use crate::security::{generate_token, hash};
use uuid::Uuid;
use validator::Validate;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

pub fn routes(context: Context) -> BoxedFilter<(impl Reply,)> {
    let moved_context = context.clone();
    let context_filter = warp::any().map(move || moved_context.clone());

    // POST /login {email, role, organization_name?} -> 200
    let login = warp::post()
        .and(warp::path!("login"))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(warp::header::<String>("user-agent"))
        .and(context_filter.clone())
        .and_then(login);

    // POST /logout -> 200
    let logout = warp::post()
        .and(warp::path!("logout"))
        .and(public_user_filter(context.clone()))
        .and(context_filter.clone())
        .and_then(logout);

    // POST /session/<session_id>/validate {confirmation_token} -> Credentials
    let session_validate = warp::post()
        .and(warp::path!("session" / Uuid / "validate"))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .and_then(validate);

    login.or(logout).or(session_validate).boxed()
}

async fn validate(
    session_id: Uuid,
    data: ValidateSessionForm,
    context: Context,
) -> Result<impl Reply, Rejection> {
    // Validate data
    if let Err(errors) = data.validate() {
        return Err(warp::reject::custom(Error::InvalidDataWithDetails {
            source: errors,
        }));
    }

    // Prepare connector
    let connector = context.builders.create();

    // Hash token
    let hashed_confirmation_token = hash(data.confirmation_token);

    // Find session
    let (session, user) =
        session::get_unconfirmed(&connector, &session_id, &hashed_confirmation_token)?;

    // Generate token and save
    let token = generate_token();
    let hashed_token = hash(token.clone());

    session::confirm(&connector, &session.id, &hashed_token)?;
    user::confirm(&connector, &session.user_id)?;
    organization::confirm(&connector, &session.user_id)?;
    checkin::confirm(&connector, &session.id)?;

    Ok(warp::reply::json(&Credentials {
        login: session.id,
        token,
        user: user.into(),
    }))
}

async fn login(
    data: LoginForm,
    user_agent: String,
    context: Context,
) -> Result<impl Reply, Rejection> {
    // TODO: Rate limit if more than 3 unconfirmed in the last 4 minutes

    // Validate data
    if let Err(errors) = data.validate() {
        return Err(warp::reject::custom(Error::InvalidDataWithDetails {
            source: errors,
        }));
    }

    // Prepare connector
    let connector = context.builders.create();

    // Get login
    let (login, stored_email) = get_auth_from_email(data.email.clone(), true);

    if !data.fallback_on_sign_up && !user::exist_with_login(&connector, &login)? {
        // Do not create user if it doesn't exist
        return Err(warp::reject::custom(Error::Unauthorized));
    }

    // Upsert user
    let user = user::insert(
        &connector,
        &user::UserInsert {
            login,
            email: stored_email,
            role: data.role,
        },
        false,
    )?;

    if data.role == user::UserRole::Professional {
        match data.organization_name {
            Some(org_name) => {
                // Upsert organization (do nothing on update)
                organization::upsert(
                    &connector,
                    &organization::OrganizationUpsert {
                        user_id: user.id,
                        name: org_name,
                        confirmed: user.confirmed,
                    },
                )?;

                if user.role != data.role {
                    // Upgrade user to pro user
                    user::update_role(&connector, user.id, data.role)?;
                }
            }
            None => return Err(warp::reject::custom(Error::InvalidData)),
        }
    }

    // Create session with confirmation token
    let session = create_session(&connector, user.id, data.email, user_agent)?;

    // Return session_id
    Ok(warp::reply::json(&session))
}

async fn logout(public: PublicUser, context: Context) -> Result<impl Reply, Rejection> {
    session::set_disabled(&context.builders.create(), &public.session.id, true)?;

    Ok(warp::reply())
}

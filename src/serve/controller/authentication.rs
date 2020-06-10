use super::super::authorization::public_user_filter;
use super::super::common::*;
use super::super::error::Error;
use super::super::types::*;
use crate::model::{organization, session, user};
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
        .and(context_filter.clone())
        .and_then(login);

    // POST /logout -> 200
    let logout = warp::post()
        .and(warp::path!("logout"))
        .and(public_user_filter(context.clone()))
        .and(context_filter.clone())
        .map(logout);

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
    let connectors = context.builders.create();

    // Hash token
    let hashed_confirmation_token = hash(data.confirmation_token);

    // Find session
    let session = session::get_unconfirmed(&connectors, &session_id, &hashed_confirmation_token)?;

    // Generate token and save
    let token = generate_token();
    let hashed_token = hash(token.clone());

    session::confirm(&connectors, &session.id, &hashed_token)?;

    Ok(warp::reply::json(&Credentials {
        login: session.id,
        token,
    }))
}

async fn login(data: LoginForm, context: Context) -> Result<impl Reply, Rejection> {
    // TODO: Rate limit if more than 3 unconfirmed in the last 4 minutes

    // Validate data
    if let Err(errors) = data.validate() {
        return Err(warp::reject::custom(Error::InvalidDataWithDetails {
            source: errors,
        }));
    }

    // Prepare connector
    let connectors = context.builders.create();

    // Hash email to get login
    let login = hash(data.email.to_lowercase());

    // Upsert user
    let user = user::insert(
        &connectors,
        &user::UserInsert {
            login,
            role: data.role,
        },
    )?;

    if user.role != data.role && data.role == user::UserRole::Professional {
        match data.organization_name {
            Some(org_name) => {
                // Upsert organization (do nothing on update)
                organization::upsert(
                    &connectors,
                    &organization::OrganizationUpsert {
                        user_id: user.id,
                        name: org_name,
                        confirmed: user.confirmed,
                    },
                )?;

                // Upgrade user to pro user
                user::update_role(&connectors, user.id, data.role)?;
            }
            None => return Err(warp::reject::custom(Error::InvalidData)),
        }
    }

    // Create session with confirmation token
    let token = generate_token();
    let session: Session = session::insert(
        &connectors,
        &session::SessionInsert {
            user_id: user.id,
            description: String::from("DESC"),
            hashed_confirmation_token: hash(token.clone()),
        },
    )?
    .into();

    // Print validation URL
    println!(
        "Validation URL: /session/{}/validate?confirmationToken={}",
        session.id, token
    );

    // Return session_id
    Ok(warp::reply::json(&session))
}

fn logout(user: PublicUser, context: Context) -> impl Reply {
    // Remove token from session
    warp::reply()
}

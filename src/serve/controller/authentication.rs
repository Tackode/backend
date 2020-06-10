use super::super::authorization::public_user_filter;
use super::super::common::*;
use super::super::error::Error;
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
        .map(validate);

    login.or(logout).or(session_validate).boxed()
}

fn validate(session_id: Uuid, data: ValidateSessionForm, context: Context) -> impl Reply {
    warp::reply::json(&Credentials {
        login: String::from("LOGIN"),
        token: String::from("TOKEN"),
    })
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
    let connector = context.builders.create();

    // Hash email to get login
    let login = hash(data.email.to_lowercase());

    // Upsert user
    let user = user::insert(
        &connector,
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
                    &connector,
                    &organization::OrganizationUpsert {
                        user_id: user.id,
                        name: org_name,
                        confirmed: user.confirmed,
                    },
                )?;

                // Upgrade user to pro user
                user::update_role(&connector, user.id, data.role)?;
            }
            None => return Err(warp::reject::custom(Error::InvalidData)),
        }
    }

    // Create session with confirmation token
    let token = generate_token();
    let session = session::insert(
        &connector,
        &session::SessionInsert {
            user_id: user.id,
            description: String::from("DESC"),
            hashed_confirmation_token: hash(token.clone()),
        },
    )?;

    // Print validation URL
    println!(
        "Validation URL: /session/{}/validate?confirmationToken={}",
        session.id, token
    );

    // Return session_id
    Ok(warp::reply::json(&Session {
        session_id: session.id,
    }))
}

fn logout(user: PublicUser, context: Context) -> impl Reply {
    // Remove token from session
    warp::reply()
}

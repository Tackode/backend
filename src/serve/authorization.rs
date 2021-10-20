use super::error::Error;
use super::types::{Context, ProfessionalUser, PublicUser};
use crate::connector::Connector;
use crate::model::session::Session;
use crate::model::{session, user};
use crate::security::hash;
use base64::decode;
use std::str::FromStr;
use tracing::debug;
use uuid::Uuid;
use warp::{reject, Filter, Rejection};

const SCHEME: &str = "Basic";

struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn public_user_filter(
    context: Context,
) -> impl Filter<Extract = (PublicUser,), Error = Rejection> + Clone {
    auth_filter(context, |connector, session| {
        user::get(connector, &session.user_id)
            .ok()
            .map(|user| PublicUser {
                session: session.into(),
                user: user.into(),
            })
    })
}

pub fn professional_user_filter(
    context: Context,
) -> impl Filter<Extract = (ProfessionalUser,), Error = Rejection> + Clone {
    auth_filter(context, |connector, session| {
        user::get_with_organization(connector, &session.user_id)
            .ok()
            .and_then(|(user, organisation)| {
                organisation.map(|organisation| ProfessionalUser {
                    session: session.into(),
                    user: user.into(),
                    organization: organisation.into(),
                })
            })
    })
}

fn auth_filter<T, F>(
    context: Context,
    get_user: F,
) -> impl Filter<Extract = (T,), Error = Rejection> + Clone
where
    F: Fn(&Connector, Session) -> Option<T> + Clone + Send,
{
    warp::header::<String>("authorization")
        .map(move |header| (header, context.clone(), get_user.clone()))
        .and_then(
            |(header, context, get_user): (String, Context, F)| async move {
                // Prepare connector
                let connector = context.builder.create();

                let user = decrypt_basic_header(header)
                    .and_then(|credentials| credentials_to_session(&connector, credentials))
                    .and_then(|session| get_user(&connector, session));

                match user {
                    Some(user) => Ok(user),
                    None => Err(reject::custom(Error::Unauthorized)),
                }
            },
        )
}

fn decrypt_basic_header(header: String) -> Option<Credentials> {
    if !header.starts_with(SCHEME) || header.len() <= SCHEME.len() + 1 {
        return None;
    }

    match header[SCHEME.len() + 1..].parse::<Credentials>() {
        Ok(h) => Some(h),
        Err(_) => None,
    }
}

fn credentials_to_session(connector: &Connector, credentials: Credentials) -> Option<Session> {
    Uuid::parse_str(&credentials.username)
        .ok()
        .map(|session_id| (session_id, hash(credentials.password)))
        .and_then(|(sid, ht)| session::get_confirmed(connector, &sid, &ht).ok())
        .flatten()
}

struct CredentialsError;

impl FromStr for Credentials {
    type Err = CredentialsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match decode(s) {
            Ok(decoded) => match String::from_utf8(decoded) {
                Ok(text) => {
                    let parts = &mut text.split(':');

                    let username = match parts.next() {
                        Some(part) => part.to_owned(),
                        None => return Err(CredentialsError),
                    };

                    let password = match parts.next() {
                        Some(part) => part.to_owned(),
                        None => return Err(CredentialsError),
                    };

                    Ok(Credentials { username, password })
                }
                Err(e) => {
                    debug!("Basic::from_utf8 error={:?}", e);
                    Err(CredentialsError)
                }
            },
            Err(e) => {
                debug!("Basic::from_base64 error={:?}", e);
                Err(CredentialsError)
            }
        }
    }
}

use super::types::Session;
use crate::connector::{email::templates::DeviceValidationEmail, Connectors};
use crate::model::error::Error;
use crate::model::session;
use crate::security::{generate_token, hash};
use uuid::Uuid;

pub fn create_session(
    connectors: &Connectors,
    user_id: Uuid,
    email_address: String,
    description: String,
) -> Result<Session, Error> {
    // Create session with confirmation token
    let token = generate_token();
    let session: Session = session::insert(
        &connectors,
        &session::SessionInsert {
            user_id,
            description,
            hashed_confirmation_token: hash(token.clone()),
        },
    )?
    .into();

    // Print validation URL
    println!(
        "Validation URL: /validate-session?sessionId={}&token={}",
        session.id, token
    );

    // TODO: Remove print and send email
    connectors
        .email
        .send(vec![DeviceValidationEmail { to: email_address }]);

    Ok(session)
}

pub fn get_auth_from_email(email: String, store_email: bool) -> (String, Option<String>) {
    // Hash email to get login
    let cleaned_email = email.to_lowercase();
    let login = hash(cleaned_email.clone());

    // Prepare stored user email
    let stored_email = if store_email {
        Some(cleaned_email)
    } else {
        None
    };

    (login, stored_email)
}

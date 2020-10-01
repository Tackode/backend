use super::types::{RedirectPage, Session};
use crate::connector::{email::template::DeviceValidationEmail, Connector};
use crate::model::error::Error;
use crate::model::session;
use crate::security::{generate_token, hash};
use uuid::Uuid;

pub fn create_session(
    connector: &Connector,
    user_id: Uuid,
    email_address: String,
    description: String,
    redirect_page: RedirectPage,
) -> Result<Session, Error> {
    // Create session with confirmation token
    let token = generate_token();
    let session: Session = session::insert(
        &connector,
        &session::SessionInsert {
            user_id,
            description,
            hashed_confirmation_token: hash(token.clone()),
        },
    )?
    .into();

    let redirect = match redirect_page {
        RedirectPage::CheckinConfirmation { place_id } => {
            format!("redirect=checkinConfirmation&placeId={}", place_id)
        }
        RedirectPage::Checkins => String::from("redirect=checkins"),
        RedirectPage::Places => String::from("redirect=places"),
    };

    // Send validation URL
    connector.email.send(vec![DeviceValidationEmail {
        to: email_address,
        url: format!(
            "/validate-session/?sessionId={}&token={}&{}",
            session.id, token, redirect
        ),
    }]);

    Ok(session)
}

pub fn get_auth_from_email(email: String) -> (String, String) {
    // Hash email to get login
    let cleaned_email = email.to_lowercase();
    let login = hash(cleaned_email.clone());

    (login, cleaned_email)
}

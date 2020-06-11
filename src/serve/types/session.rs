use crate::model::session::Session as SessionModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValidateSessionForm {
    #[validate(length(equal = 128))]
    pub confirmation_token: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: Uuid,
    pub confirmed: bool,
}

#[derive(Serialize)]
pub struct Credentials {
    pub login: Uuid,
    pub token: String,
}

impl From<SessionModel> for Session {
    fn from(session: SessionModel) -> Self {
        Session {
            id: session.id,
            confirmed: session.confirmed,
        }
    }
}

use super::Organization;
use crate::model::organization::Organization as OrganizationModel;
use crate::model::user::{User as UserModel, UserRole};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: Option<String>,
}

#[derive(Serialize)]
pub struct Profile {
    pub id: Uuid,
    pub email: Option<String>,
    pub organization: Option<Organization>,
}

#[derive(Deserialize, Validate)]
pub struct ProfileForm {
    #[validate(email)]
    pub email: Option<String>,
}

impl From<UserModel> for User {
    fn from(user: UserModel) -> Self {
        User {
            id: user.id,
            email: user.email,
        }
    }
}

impl From<(UserModel, Option<OrganizationModel>)> for Profile {
    fn from((user, org): (UserModel, Option<OrganizationModel>)) -> Self {
        Profile {
            id: user.id,
            email: user.email,
            organization: org.map(|o| o.into()),
        }
    }
}

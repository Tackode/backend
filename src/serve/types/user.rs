use super::{Organization, Session};
use crate::model::organization::Organization as OrganizationModel;
use crate::model::user::{User as UserModel, UserRole};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub role: UserRole,
    pub email: String,
}

pub struct PublicUser {
    pub user: User,
    pub session: Session,
}

pub struct ProfessionalUser {
    pub user: User,
    pub session: Session,
    pub organization: Organization,
}

#[derive(Serialize)]
pub struct Profile {
    pub id: Uuid,
    pub role: UserRole,
    pub email: String,
    pub organization: Option<Organization>,
}

#[derive(Deserialize, Validate)]
pub struct ProfileForm {
    #[validate(email)]
    pub email: String,
}

impl From<UserModel> for User {
    fn from(user: UserModel) -> Self {
        User {
            id: user.id,
            role: user.role,
            email: user.email,
        }
    }
}

impl From<(UserModel, Option<OrganizationModel>)> for Profile {
    fn from((user, org): (UserModel, Option<OrganizationModel>)) -> Self {
        Profile {
            id: user.id,
            role: user.role,
            email: user.email,
            organization: org.map(|o| o.into()),
        }
    }
}

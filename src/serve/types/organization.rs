use crate::model::organization::Organization as OrganizationModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct OrganizationForm {
    #[validate(length(min = 1, max = 60))]
    pub name: String,
}

impl From<OrganizationModel> for Organization {
    fn from(org: OrganizationModel) -> Self {
        Organization {
            id: org.id,
            name: org.name,
        }
    }
}

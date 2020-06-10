use crate::model::organization::Organization as OrganizationModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize)]
pub struct OrganizationForm {
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

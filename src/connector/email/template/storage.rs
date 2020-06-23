use super::device_validation::DeviceValidationTemplate;
use super::infection_warning::InfectionWarningTemplate;
use std::env;

#[derive(Clone)]
pub struct TemplateStorage {
    pub front_public_url: String,
    pub device_validation: DeviceValidationTemplate,
    pub infection_warning: InfectionWarningTemplate,
}

impl TemplateStorage {
    pub fn new() -> Self {
        let front_public_url = env::var("FRONT_PUBLIC_URL").expect("Missing FRONT_PUBLIC_URL");

        TemplateStorage {
            front_public_url,
            device_validation: DeviceValidationTemplate::new(),
            infection_warning: InfectionWarningTemplate::new(),
        }
    }
}

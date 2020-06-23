use super::device_validation::DeviceValidationTemplate;
use std::env;

#[derive(Clone)]
pub struct TemplateStorage {
    pub front_public_url: String,
    pub device_validation: DeviceValidationTemplate,
}

impl TemplateStorage {
    pub fn new() -> Self {
        let front_public_url = env::var("FRONT_PUBLIC_URL").expect("Missing FRONT_PUBLIC_URL");

        TemplateStorage {
            front_public_url,
            device_validation: DeviceValidationTemplate::new(),
        }
    }
}

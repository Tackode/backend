use super::{
    precompile_template, EmailData, EmailTemplate, PrecompiledTemplate, TemplateData,
    TemplateStorage,
};
use std::collections::HashMap;

pub struct DeviceValidationEmail {
    pub to: String,
    pub url: String,
}

impl EmailData for DeviceValidationEmail {
    fn to(&self) -> String {
        self.to.clone()
    }

    fn template_from_storage(storage: &TemplateStorage) -> &dyn EmailTemplate {
        &storage.device_validation
    }

    fn into(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("url".to_string(), self.url.clone());

        map
    }
}

#[derive(Clone)]
pub struct DeviceValidationTemplate {
    precompiled: PrecompiledTemplate,
}

impl DeviceValidationTemplate {
    pub fn new() -> Self {
        DeviceValidationTemplate {
            precompiled: precompile_template(TemplateData {
                name: "deviceValidation",
                subject: "Validation de la session",
                utf8_subject: false,
                embeds: vec![(
                    "../assets/logo.png",
                    "image/png".parse().expect("Unable to parse ContentType"),
                )],
            }),
        }
    }
}

impl EmailTemplate for DeviceValidationTemplate {
    fn precompiled(&self) -> &PrecompiledTemplate {
        &self.precompiled
    }
}

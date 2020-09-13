use super::{
    precompile_template, EmailData, EmailTemplate, PrecompiledTemplate, TemplateData,
    TemplateStorage,
};
use chrono::{DateTime, Utc};
use chrono_tz::Europe::Paris;
use lettre::message::mime;
use std::collections::HashMap;

pub struct InfectionWarningEmail {
    pub to: String,
    pub organization_name: String,
    pub place_name: String,
    pub checkin_datetime: DateTime<Utc>,
}

impl EmailData for InfectionWarningEmail {
    fn to(&self) -> String {
        self.to.clone()
    }

    fn template_from_storage(storage: &TemplateStorage) -> &dyn EmailTemplate {
        &storage.infection_warning
    }

    fn into(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert(
            "organizationName".to_string(),
            self.organization_name.clone(),
        );
        map.insert("placeName".to_string(), self.place_name.clone());

        let paris_time = self.checkin_datetime.with_timezone(&Paris);
        map.insert(
            "checkinDateTime".to_string(),
            paris_time.format("%d/%m/%y à %Hh%M").to_string(),
        );

        map
    }
}

#[derive(Clone)]
pub struct InfectionWarningTemplate {
    precompiled: PrecompiledTemplate,
}

impl InfectionWarningTemplate {
    pub fn new() -> Self {
        InfectionWarningTemplate {
            precompiled: precompile_template(TemplateData {
                name: "infectionWarning",
                subject: "⚠️ Contact potentiel avec une personne infectée",
                utf8_subject: true,
                embeds: vec![("../assets/logo.png", mime::IMAGE_PNG)],
            }),
        }
    }
}

impl EmailTemplate for InfectionWarningTemplate {
    fn precompiled(&self) -> &PrecompiledTemplate {
        &self.precompiled
    }
}

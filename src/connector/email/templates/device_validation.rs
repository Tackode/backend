use super::{embed_in_template, template, Email, EmailTemplate};
use lettre_email::mime;

pub struct DeviceValidationEmail {
    pub to: String,
}

// impl EmailData<DeviceValidationBuilder> for DeviceValidationEmail

impl EmailTemplate for DeviceValidationEmail {
    fn build(&self) -> Option<Email> {
        template("deviceValidation.html")
            .and_then(|template| embed_in_template(template, "../images/logo.png", mime::IMAGE_PNG))
            .map(|(template, embed)| Email {
                to: self.to.clone(),
                subject: String::from("Covid-Journal - Validation de la session"),
                body: template,
                embeds: vec![embed],
            })
    }
}

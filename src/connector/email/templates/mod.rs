mod device_validation;

pub use device_validation::DeviceValidationEmail;

use lettre_email::mime::Mime;
use rust_embed::RustEmbed;
use std::path::Path;

#[derive(RustEmbed)]
#[folder = "emails/"]
struct Asset;

fn template(filename: &str) -> Option<String> {
    Asset::get(filename).and_then(|template| String::from_utf8(template.into()).ok())
}

fn embed_in_template(
    template: String,
    filepath: &str,
    content_type: Mime,
) -> Option<(String, Embed)> {
    let path = Path::new(filepath);

    // Find filename
    path.file_name()
        .and_then(|file_name| file_name.to_str())
        .map(|file_name| file_name.to_string())
        .and_then(|filename| {
            // Load file
            Asset::get(filepath).map(|body| {
                // Replace references in template by content id
                let content_id = filename.clone(); // format!("{}@covid-journal.org", filename);
                let content_id_tmpl = format!("cid:{}", content_id);
                let new_template = template.replace(filepath, content_id_tmpl.as_str());

                (
                    new_template,
                    Embed {
                        body: body.into(),
                        filename,
                        content_type,
                        content_id: content_id.to_string(),
                    },
                )
            })
        })
}

pub struct Embed {
    pub body: Vec<u8>,
    pub filename: String,
    pub content_type: Mime,
    pub content_id: String,
}

pub struct Email {
    pub to: String,
    pub subject: String,
    pub body: String,
    pub embeds: Vec<Embed>,
}

pub trait EmailTemplate {
    fn build(&self) -> Option<Email>;
}

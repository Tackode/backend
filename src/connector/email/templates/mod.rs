mod device_validation;

pub use device_validation::DeviceValidationEmail;

use lettre_email::mime::Mime;
use rust_embed::RustEmbed;
use std::path::Path;

#[derive(RustEmbed)]
#[folder = "emails/html/"]
struct Template;

#[derive(RustEmbed)]
#[folder = "emails/images/"]
struct Image;

fn template(filename: &str) -> Option<String> {
    Template::get(filename).and_then(|template| String::from_utf8(template.into()).ok())
}

fn embed_in_template(
    template: String,
    filepath: &str,
    content_type: Mime,
) -> Option<(String, Embed)> {
    get_embed_filepath(filepath)
        .and_then(|embed_filepath| {
            get_filename(filepath).map(|filename| (embed_filepath, filename))
        })
        .and_then(|(embed_filepath, filename)| // Load file
            Image::get(&embed_filepath).map(|body| {
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
            }))
}

fn get_embed_filepath(filepath: &str) -> Option<String> {
    Path::new(filepath)
        .strip_prefix("../images")
        .ok()
        .and_then(|f| f.to_str())
        .map(|f| f.to_string())
}

fn get_filename(filepath: &str) -> Option<String> {
    Path::new(filepath)
        .file_name()
        .and_then(|f| f.to_str())
        .map(|f| f.to_string())
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

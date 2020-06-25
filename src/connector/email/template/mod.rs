mod device_validation;
mod infection_warning;
mod storage;

use lettre_email::mime::Mime;
use rust_embed::RustEmbed;
use std::collections::HashMap;
use std::path::Path;

pub use device_validation::DeviceValidationEmail;
pub use infection_warning::InfectionWarningEmail;
pub use storage::TemplateStorage;

#[derive(RustEmbed)]
#[folder = "emails/html/"]
struct Html;

#[derive(RustEmbed)]
#[folder = "emails/text/"]
struct Text;

#[derive(RustEmbed)]
#[folder = "emails/assets/"]
struct Asset;

pub struct TemplateData {
    pub name: &'static str,
    pub subject: &'static str,
    pub utf8_subject: bool,
    pub embeds: Vec<(&'static str, Mime)>,
}

#[derive(Clone)]
pub struct PrecompiledTemplate {
    name: String,
    html: String,
    text: String,
    subject: String,
    utf8_subject: bool,
    embeds: Vec<Embed>,
}

#[derive(Clone)]
pub struct Embed {
    pub body: Vec<u8>,
    pub filename: String,
    pub content_type: Mime,
    pub content_id: String,
}

pub trait EmailTemplate {
    fn precompiled(&self) -> &PrecompiledTemplate;
}

pub struct CompiledEmail {
    pub to: String,
    pub html: String,
    pub text: String,
    pub subject: String,
    pub embeds: Vec<Embed>,
}

fn precompile_template(data: TemplateData) -> PrecompiledTemplate {
    // Expect template html and txt
    let mut html = html(format!("{}.html", data.name))
        .expect(&format!("{} HTML template not found", data.name));

    let text = text(format!("{}.txt", data.name))
        .expect(&format!("{} Text template not found", data.name));

    // Prepare and replace embeds
    let embeds =
        data.embeds
            .iter()
            .map(|(filepath, content_type)| {
                let (new_html, embed) =
                    embed_in_template(html.clone(), filepath, content_type.clone()).expect(
                        &format!("{} Embed not found in template {}", filepath, data.name),
                    );

                // Assign prepared HTML
                html = new_html;

                // Return prepared embed
                embed
            })
            .collect();

    PrecompiledTemplate {
        name: data.name.to_string(),
        html: html.clone(),
        text: text.clone(),
        subject: data.subject.to_string(),
        utf8_subject: data.utf8_subject,
        embeds,
    }
}

pub trait EmailData {
    fn to(&self) -> String;
    fn template_from_storage(storage: &TemplateStorage) -> &dyn EmailTemplate;
    fn into(&self) -> HashMap<String, String>;

    fn compile_with(&self, storage: &TemplateStorage) -> CompiledEmail {
        let mut data: HashMap<String, String> = self.into();
        data.insert(
            "frontPublicUrl".to_string(),
            storage.front_public_url.clone(),
        );

        Self::template_from_storage(storage)
            .precompiled()
            .compile(self.to(), data)
    }
}

impl PrecompiledTemplate {
    fn compile(&self, to: String, data: HashMap<String, String>) -> CompiledEmail {
        // Replace all keys with values in html, text and subject
        let mut html = self.html.clone();
        let mut text = self.text.clone();
        let mut subject = self.subject.clone();

        for (key, value) in data {
            let key = format!("{{{{{}}}}}", key);

            html = html.replace(&key, &value);
            text = text.replace(&key, &value);
            subject = subject.replace(&key, &value);
        }

        if self.utf8_subject {
            subject = format!("=?UTF-8?B?{}?=", base64::encode(subject))
        }

        CompiledEmail {
            to,
            html,
            text,
            subject,
            embeds: self.embeds.clone(),
        }
    }
}

fn html(filename: String) -> Option<String> {
    Html::get(&filename).and_then(|template| String::from_utf8(template.into()).ok())
}

fn text(filename: String) -> Option<String> {
    Text::get(&filename).and_then(|template| String::from_utf8(template.into()).ok())
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
            Asset::get(&embed_filepath).map(|body| {
                // Replace references in template by content id
                let content_id = filename.clone(); // format!("{}@covid-journal.org", filename);
                let content_id_tmpl = format!("cid:{}", content_id);
                let new_template = template.replace(filepath, &content_id_tmpl);

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
        .strip_prefix("../assets")
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

pub mod template;

use lettre::message::{header, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::extension::ClientId;
use lettre::{Address, Mailbox, Message, SmtpTransport, Transport};
use std::{env, str::FromStr};
use template::{EmailData, TemplateStorage};

pub struct Connector {
    smtp_transport: SmtpTransport,
    smtp_domain: String,
    from: Mailbox,
    template_storage: TemplateStorage,
}

impl Connector {
    // Use EmailData to instanciate an email
    pub fn send(&self, data: Vec<impl EmailData>) {
        for data in data.iter() {
            match data.compile_with(&self.template_storage) {
                Ok(email) => {
                    let html_part = MultiPart::related().singlepart(
                        SinglePart::quoted_printable()
                            .header(header::ContentType(
                                "text/html; charset=utf8".parse().unwrap(),
                            ))
                            .body(email.html.clone()),
                    );

                    // Handle embeds
                    let html_part = email.embeds.iter().fold(html_part, |html_part, embed| {
                        html_part.singlepart(
                            SinglePart::base64()
                                .header(header::ContentType(embed.content_type.clone()))
                                .header(header::ContentDisposition {
                                    disposition: header::DispositionType::Inline,
                                    parameters: vec![],
                                })
                                .header(header::ContentId(format!("<{}>", embed.content_id)))
                                .body(embed.body.clone()),
                        )
                    });

                    let message_id = format!("<{}@{}>", uuid::Uuid::new_v4(), self.smtp_domain);

                    let message = Message::builder()
                        .to(Mailbox::new(None, email.to))
                        .from(self.from.clone())
                        .subject(email.subject.clone())
                        .message_id(Some(message_id))
                        .multipart(
                            MultiPart::alternative()
                                .singlepart(
                                    SinglePart::quoted_printable()
                                        .header(header::ContentType(
                                            "text/plain; charset=utf8".parse().unwrap(),
                                        ))
                                        .body(email.text),
                                )
                                .multipart(html_part),
                        );

                    match message {
                        Ok(message) => match self.smtp_transport.send(&message) {
                            Ok(_) => (),
                            Err(error) => log::error!("Error while sending email: {}", error),
                        },
                        Err(error) => log::error!("Error while building email: {}", error),
                    }
                }

                Err(error) => log::error!("Error while compiling email: {}", error),
            }
        }
    }
}

#[derive(Clone)]
pub struct ConnectorBuilder {
    smtp_transport: SmtpTransport,
    smtp_domain: String,
    from: Mailbox,
    template_storage: TemplateStorage,
}

impl ConnectorBuilder {
    pub fn new() -> ConnectorBuilder {
        let smtp_domain = env::var("EMAIL_SMTP_DOMAIN").expect("EMAIL_SMTP_DOMAIN must be set");
        let smtp_login = env::var("EMAIL_SMTP_LOGIN").expect("EMAIL_SMTP_LOGIN must be set");
        let smtp_password =
            env::var("EMAIL_SMTP_PASSWORD").expect("EMAIL_SMTP_PASSWORD must be set");
        let from_name = env::var("EMAIL_FROM_NAME").expect("EMAIL_FROM_NAME must be set");
        let from_address = env::var("EMAIL_FROM_ADDRESS")
            .map(|add| Address::from_str(&add).expect("EMAIL_FROM_ADDRESS must be a valid address"))
            .expect("EMAIL_FROM_ADDRESS must be set");

        // Prepare From
        let from = Mailbox::new(Some(from_name), from_address);

        // Prepare SMTP client
        let smtp_transport = SmtpTransport::starttls_relay(&smtp_domain)
            .expect("Unable to create SMTP relay")
            .hello_name(ClientId::Domain(smtp_domain.clone()))
            .credentials(Credentials::new(smtp_login, smtp_password))
            .build();

        ConnectorBuilder {
            smtp_transport,
            smtp_domain,
            from,
            template_storage: TemplateStorage::new(),
        }
    }

    pub fn create(&self) -> Connector {
        Connector {
            smtp_transport: self.smtp_transport.clone(),
            smtp_domain: self.smtp_domain.clone(),
            from: self.from.clone(),
            template_storage: self.template_storage.clone(),
        }
    }
}

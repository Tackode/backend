pub mod templates;

use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::extension::ClientId;
use lettre::smtp::ConnectionReuseParameters;
use lettre::{ClientSecurity, ClientTlsParameters, SmtpClient, Transport};
use lettre_email::{Email, MimeMultipartType, PartBuilder};
use native_tls::{Protocol, TlsConnector};
use std::env;
use templates::EmailTemplate;

// pub struct Email {
//     pub to: String
//     pub model: EmailModel
// }

// Create Connector with smtp data and from
// Create email as mjml and build it in a folder
// Describe meta data in struct
// Create send method
//   - Find html email in embed
//   - Use data to build it
//   - Add images embed
//   - Send email
// Send email to validate session
// Create send to multiple
// Send email to multiple infected people with connection reuse

pub struct Connector {
    smtp_client: SmtpClient,
    from_name: String,
    from_address: String,
}

impl Connector {
    // Use EmailData to instanciate an email
    pub fn send(&self, templates: Vec<impl EmailTemplate>) {
        let mut mailer = self.smtp_client.clone().transport();

        for template in templates.iter() {
            template
                .build()
                .and_then(|email| {
                    let builder = Email::builder()
                        .to(email.to)
                        .from((self.from_address.clone(), self.from_name.clone()))
                        .subject(email.subject)
                        .html(email.body);

                    // Handle embeds
                    let builder = email.embeds.iter().fold(builder, |builder, embed| {
                        let encoded_body = base64::encode(&embed.body);
                        let content = PartBuilder::new()
                            .body(encoded_body)
                            .header((
                                "Content-Disposition",
                                format!("attachment; filename=\"{}\"", embed.filename),
                            ))
                            .header((
                                "Content-Type",
                                format!("{}; name=\"{}\"", embed.content_type, embed.filename),
                            ))
                            .header(("Content-Transfer-Encoding", "base64"))
                            .header(("Content-ID", format!("<{}>", embed.content_id)))
                            .build();

                        builder
                            .message_type(MimeMultipartType::Mixed)
                            .child(content)
                    });

                    builder.build().ok()
                })
                .and_then(|email| mailer.send(email.into()).ok());
        }

        // Explicitly close the SMTP transaction as we enabled connection reuse
        mailer.close();
    }
}

#[derive(Clone)]
pub struct ConnectorBuilder {
    smtp_client: SmtpClient,
    from_name: String,
    from_address: String,
}

impl ConnectorBuilder {
    pub fn new() -> ConnectorBuilder {
        let smtp_server = env::var("EMAIL_SMTP_SERVER").expect("Missing EMAIL_SMTP_SERVER");
        let smtp_domain = env::var("EMAIL_SMTP_DOMAIN").expect("EMAIL_SMTP_DOMAIN must be set");
        let smtp_login = env::var("EMAIL_SMTP_LOGIN").expect("EMAIL_SMTP_LOGIN must be set");
        let smtp_password =
            env::var("EMAIL_SMTP_PASSWORD").expect("EMAIL_SMTP_PASSWORD must be set");
        let from_name = env::var("EMAIL_FROM_NAME").expect("EMAIL_FROM_NAME must be set");
        let from_address = env::var("EMAIL_FROM_ADDRESS").expect("EMAIL_FROM_ADDRESS must be set");

        // Prepare TLS
        let mut tls_builder = TlsConnector::builder();
        tls_builder.min_protocol_version(Some(Protocol::Tlsv12));

        let tls_parameters =
            ClientTlsParameters::new(smtp_domain.clone(), tls_builder.build().unwrap());

        // Prepare SMTP client
        let smtp_client = SmtpClient::new(smtp_server, ClientSecurity::Required(tls_parameters))
            .expect("Cannot create SMTP client")
            .hello_name(ClientId::Domain(smtp_domain))
            .authentication_mechanism(Mechanism::Login)
            .credentials(Credentials::new(smtp_login, smtp_password))
            .smtp_utf8(true)
            .connection_reuse(ConnectionReuseParameters::ReuseUnlimited);

        // Init templates
        // Load HTML and TXT
        // Prepare content_id in HTML

        ConnectorBuilder {
            smtp_client,
            from_name,
            from_address,
        }
    }

    pub fn create(&self) -> Connector {
        Connector {
            smtp_client: self.smtp_client.clone(),
            from_name: self.from_name.clone(),
            from_address: self.from_address.clone(),
        }
    }
}

use anyhow::Result;
use handlebars::Handlebars;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;
use lettre::{Message, SmtpTransport, Transport};

pub fn get_mailer() -> Result<SmtpTransport> {
    Ok(SmtpTransport::relay(&std::env::var("SMTP_HOST")?)?
        .credentials(Credentials::new(
            std::env::var("SMTP_USER")?,
            std::env::var("SMTP_PASS")?,
        ))
        .build())
}

pub async fn send_email(
    to: &str,
    template_file: &str,
    subject: &str,
    values: &str,
) -> Result<Response> {
    // TODO: use a global Handlebars instance and load all templates at start
    let mut hb = Handlebars::new();
    hb.register_template_file("template", template_file)?;
    let output = hb.render_template("template", &values)?;

    let message = Message::builder()
        .from(std::env::var("SMTP_FROM")?.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(output)?;

    let mailer = get_mailer()?;
    Ok(mailer.send(&message)?)
}

use anyhow::{Error, Result};
use chrono::{NaiveDate, NaiveTime};
use handlebars::Handlebars;
use lettre::{
    transport::smtp::{authentication::Credentials, response::Response},
    Message, SmtpTransport, Transport,
};

use std::collections::HashMap;

//TODO: we can apply these with serde during deserialization
pub fn parse_form_time(time: &str) -> Result<NaiveTime> {
    Ok(NaiveTime::parse_from_str(time, "%H:%M")?)
}

pub fn parse_form_date(date: &str) -> Result<NaiveDate> {
    Ok(NaiveDate::parse_from_str(date, "%m/%d/%Y")?)
}

pub fn smtp_enabled() -> bool {
    (std::env::var("USE_SMTP").unwrap_or("false".to_string())).to_lowercase() == "true"
}

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
    values: HashMap<&str, &str>,
) -> Result<Response> {
    // TODO: use a global Handlebars instance and load all templates at start
    if !smtp_enabled() {
        return Err(Error::msg("SMTP is disabled"));
    }
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

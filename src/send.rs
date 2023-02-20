use lettre::{transport::smtp::SmtpTransport, Message, Transport};

pub fn send_mail(
    transport: SmtpTransport,
    from: String,
    to: String,
    subject: String,
    body: String,
) -> anyhow::Result<()> {
    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body)?;

    transport.send(&email)?;

    Ok(())
}

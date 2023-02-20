use crate::read::read_mail;
use crate::send::send_mail;
use imap;
use lettre::{
    transport::smtp::authentication::{Credentials, Mechanism},
    SmtpTransport,
};
use mail_parser::decoders;
use native_tls::TlsConnector;
use spinners::{Spinner, Spinners};
use std::io::Write;

mod cli;
mod read;
mod send;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = cli::Cli::new().parse_all();

    match args.commands {
        Some(cli::Command::Send(send_args)) => {
            let mut spinner = Spinner::new(Spinners::Dots9, "Sending email... ".to_string());

            let smtp_transport = if send_args.tls_enabled {
                SmtpTransport::starttls_relay(send_args.server.unwrap().as_str())?
                    .credentials(Credentials::new(
                        send_args.username.unwrap(),
                        send_args.password.unwrap(),
                    ))
                    .authentication(vec![Mechanism::Plain])
                    .port(587)
                    .build()
            } else {
                SmtpTransport::relay(send_args.server.unwrap().as_str())?
                    .port(send_args.port.unwrap())
                    .build()
            };

            send_mail(
                smtp_transport,
                send_args.from,
                send_args.to,
                send_args.subject.to_string(),
                send_args.body.to_string(),
            )?;

            spinner.stop();
        }

        Some(cli::Command::Read(read_args)) => {
            let mut spinner = Spinner::new(Spinners::Dots9, "Fetching emails... ".to_string());
            let imap_client = {
                let tls = TlsConnector::builder().build()?;

                imap::connect(
                    (
                        read_args.server.clone().unwrap().as_str(),
                        read_args.port.unwrap(),
                    ),
                    read_args.server.unwrap().as_str(),
                    &tls,
                )?
            };

            let imap_session = imap_client
                .login(read_args.username.unwrap(), read_args.password.unwrap())
                .map_err(|e| e.0)?;

            let messages = read_mail(imap_session, read_args.mailbox, read_args.count.unwrap())?;

            if let Some(output) = read_args.output {
                let mut file = std::fs::File::create(output)?;

                for message in messages.iter() {
                    let body = message.body().or_else(|| {
                        "Message has no body".to_string();

                        None
                    });

                    if let Some(body) = body {
                        file.write_all(&decoders::base64::base64_decode(body).unwrap())?;
                    } else {
                        continue;
                    }
                }
            } else {
                for message in messages.iter() {
                    let body = message.body().or_else(|| {
                        "Message has no body".to_string();

                        None
                    });

                    if let Some(body) = body {
                        println!("Message body: {}\n\n\n", std::str::from_utf8(body)?);
                        println!(
                            "{}\n",
                            decoders::html::html_to_text(std::str::from_utf8(body)?)
                        );
                    } else {
                        continue;
                    }
                }
            }

            spinner.stop();
        }

        _ => {
            panic!("Invalid command provided. Expects either 'send' or 'read'.");
        }
    };

    Ok(())
}

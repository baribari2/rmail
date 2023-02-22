use crate::read::read_mail;
use crate::send::send_mail;
use lettre::{
    transport::smtp::authentication::{Credentials, Mechanism},
    SmtpTransport,
};
use native_tls::TlsConnector;
use spinners::{Spinner, Spinners};

mod cli;
mod read;
mod send;

fn main() -> anyhow::Result<()> {
    let args = cli::Cli::new().parse_all();

    match args.commands {
        // Send command
        Some(cli::Command::Send(send_args)) => {
            let mut spinner = Spinner::new(Spinners::Dots9, "📧 Sending email... ".to_string());

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
                send_args.body,
            )?;
            spinner.stop();
        }

        // Read command
        Some(cli::Command::Read(read_args)) => {
            let mut spinner = Spinner::new(Spinners::Dots9, "📧 Fetching emails...".to_string());
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

            read_mail(
                imap_session,
                read_args.mailbox,
                read_args.count.unwrap(),
                read_args.output,
            )?;

            spinner.stop();
        }

        _ => {
            panic!("Invalid command provided. Expects either 'send' or 'read'.");
        }
    };

    Ok(())
}

use clap::{Args, Parser, Subcommand};
use std::env;

#[derive(Parser, Debug, Clone)]
#[command(
    name("cli"),
    about("A CLI utility for sending and reading emails"),
    long_about("A CLI utility for sending emails over SMTP and reading emails over IMAP."),
    version("0.1.0")
)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Option<Command>,
}

impl Cli {
    pub fn new() -> Self {
        Self { commands: None }
    }

    pub fn parse_all(self) -> Cli {
        let mut args = Self::parse();

        match args.clone().commands {
            Some(Command::Send(mut send_args)) => {
                if send_args.server == None {
                    send_args.server = Some(env::var("SERVER").unwrap());
                }

                if send_args.port == None {
                    send_args.port = Some(env::var("PORT").unwrap().parse().unwrap());
                }

                if send_args.username == None {
                    send_args.username = Some(env::var("USERNAME").unwrap());
                }

                if send_args.password == None {
                    send_args.password = Some(env::var("PASSWORD").unwrap());
                };

                args.commands = Some(Command::Send(send_args));

                &args
            }

            Some(Command::Read(mut read_args)) => {
                if read_args.server == None {
                    read_args.server = Some(env::var("SERVER").unwrap());
                }

                if read_args.port == None {
                    read_args.port = Some(env::var("PORT").unwrap().parse().unwrap());
                }

                if read_args.username == None {
                    read_args.username = Some(env::var("USERNAME").unwrap());
                }

                if read_args.password == None {
                    read_args.password = Some(env::var("PASSWORD").unwrap());
                }

                args.commands = Some(Command::Read(read_args));

                &args
            }

            _ => {
                println!("No command provided");

                &args
            }
        };

        args
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Send an email
    #[command(name("send"))]
    Send(SendArgs),

    /// Read emails
    #[command(name("read"))]
    Read(ReadArgs),
}

#[derive(Args, Debug, Clone)]
pub struct SendArgs {
    /// Indicates whether or not to use TLS
    #[arg(short('e'), long)]
    pub tls_enabled: bool,

    /// The SMTP sever to send the email from
    #[arg(short('v'), long)]
    pub server: Option<String>,

    /// The port to send the email from
    #[arg(short, long)]
    pub port: Option<u16>,

    /// The username to send the email from
    #[arg(short, long)]
    pub username: Option<String>,

    /// The password to send the email from
    #[arg(short('w'), long)]
    pub password: Option<String>,

    /// The email address to send the email from
    #[arg(short, long)]
    pub from: String,

    /// The email address to send the email to
    #[arg(short, long)]
    pub to: String,

    /// The subject of the email being sent
    #[arg(short, long)]
    pub subject: String,

    /// The body of the email being sent
    #[arg(short, long)]
    pub body: String,
}

#[derive(Args, Debug, Clone)]
pub struct ReadArgs {
    /// The IMAP sever to read the emails from
    #[arg(short, long)]
    pub server: Option<String>,

    /// The port to read the emails from
    #[arg(short, long)]
    pub port: Option<u16>,

    /// The username to read the emails from
    #[arg(short, long)]
    pub username: Option<String>,

    /// The password to read the emails from
    #[arg(short('w'), long)]
    pub password: Option<String>,

    /// The mailbox from which to fetch emails
    #[arg(short, long)]
    pub mailbox: String,

    /// The number of emails to fetch (default: 100)
    #[arg(short, long, default_value = "100")]
    pub count: Option<u32>,

    /// The file path to output the emails to (default: stdout)
    #[arg(short, long)]
    pub output: Option<String>,
}

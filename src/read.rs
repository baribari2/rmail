use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use imap::{
    types::{Fetch, ZeroCopy},
    Session,
};
use mailparse::MailHeaderMap;
use std::io::{stdout, Read, Write};

pub fn read_mail<T: Write + Read>(
    mut session: Session<T>,
    mailbox: String,
    count: u32,
    output: Option<String>,
) -> anyhow::Result<ZeroCopy<Vec<Fetch>>> {
    session.select(mailbox)?;

    let m = session.fetch(format!("1:{count}"), "RFC822")?;

    if let Some(out) = output {
        let mut file = std::fs::File::create(out).expect("Failed to create file");

        for message in m.iter() {
            if let Some(body) = message.body() {
                if let Ok(data) = mailparse::parse_mail(body) {
                    if let Err(e) = file.write_all(data.subparts[0].get_body().unwrap().as_bytes())
                    {
                        println!("Failed to write to file: {}", e);

                        continue;
                    }
                };
            } else {
                println!("Message has no body");

                continue;
            }
        }
    } else {
        execute!(stdout(), Clear(ClearType::Purge))?;

        for message in m.iter() {
            if let Some(body) = message.body() {
                if let Ok(msg) = mailparse::parse_mail(body) {
                    println!(
                        "\n--------------------------------------------------------------------------------\n\x1b[32m{}\x1b[0m\n--------------------------------------------------------------------------------",
                        msg.headers.get_first_value("Subject").unwrap(),
                    );
                    println!("{}\n", msg.subparts[0].get_body().unwrap());
                } else {
                    println!("Failed to parse message:");

                    continue;
                }
            } else {
                println!("Message has no body");

                continue;
            }
        }
    }

    Ok(m)
}

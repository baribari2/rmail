use imap::{
    types::{Fetch, ZeroCopy},
    Session,
};
use std::io::{Read, Write};

pub fn read_mail<T: Write + Read>(
    mut session: Session<T>,
    mailbox: String,
    count: u32,
) -> anyhow::Result<ZeroCopy<Vec<Fetch>>> {
    session.select(mailbox)?;

    let m = session.fetch(format!("1:{count}"), "RFC822")?;

    Ok(m)
}

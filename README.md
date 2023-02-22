# rmail

rmail is a CLI utility for sending emails over SMTP and reading emails over IMAP (and soon POP3).

# Installation


# Configuration
Configuration instructions for several email clients are provided below:
- **Gmail**
    - [SMTP](https://support.google.com/mail/answer/7126229?hl=en#zippy=%2Cstep-change-smtp-other-settings-in-your-email-client%2Cstep-check-that-imap-is-turned-on)
    - [IMAP](https://support.google.com/mail/answer/7126229?hl=en#zippy=%2Cstep-change-smtp-other-settings-in-your-email-client%2Cstep-check-that-imap-is-turned-on%2Ci-cant-sign-in-to-my-email-client)
- **ProtonMail**
    - [SMTP](https://proton.me/support/imap-smtp-and-pop3-setup)
    - [IMAP](https://proton.me/support/imap-smtp-and-pop3-setup)
- **Outlook**
    - [SMTP](https://support.microsoft.com/en-us/office/pop-imap-and-smtp-settings-for-outlook-com-d088b986-291d-42b8-9564-9c414e2aa040)
    - [IMAP](https://support.microsoft.com/en-us/office/pop-imap-and-smtp-settings-for-outlook-com-d088b986-291d-42b8-9564-9c414e2aa040)
- **Apple**
    - [SMTP](https://support.apple.com/en-us/HT202304)
    - [IMAP](https://support.apple.com/en-us/HT202304)
- **GoDaddy**
    - [SMTP](https://www.godaddy.com/help/server-and-port-settings-for-workspace-email-6949)
    - [IMAP](https://www.godaddy.com/help/server-and-port-settings-for-workspace-email-6949)


# Usage
 If your email client isn't already configured, you will have to do so - steps for various clients can be found above.

```
Usage: rmail [COMMAND]

Commands:
  send
          Send an email
  read
          Read emails
  help
          Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
> **Note**
> You can use the --help flag on any subcommand to see a more detailed explanation of that command.

You can specify several configuration values as environment variables. These values are `SERVER`, `PORT`, `USERNAME`, and `PASSWORD` and are not needed as CLI flags when set. In the case that both are provided, the environment variables will take precedence.

# Develop
- Make sure you have the latest of rust (see [rustup](https://rustup.rs/))
- Run `cargo build`

# Resources
- [RFC822 (Arpa Internet Text Messages)](https://www.w3.org/Protocols/rfc822/)
- [RFC5321 (SMTP)](https://www.rfc-editor.org/rfc/rfc5321)
- [RFC3501 (IMAP)](https://www.rfc-editor.org/rfc/rfc9051)
- [RFC5037 (POP3)](https://datatracker.ietf.org/doc/html/rfc5034)

# Coming Soon
- [ ] Prettier output
- [ ] POP3 Support

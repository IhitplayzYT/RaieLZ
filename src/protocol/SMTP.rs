pub mod smtp{
    use std::error::Error;
use lettre::Message;
    use lettre::SmtpTransport;
    use lettre::Transport;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;

use crate::config::Config::EmailAccount;

    pub fn send_mail(mail: &EmailAccount,recipient: &str,subj: &str,body: &str) -> Result<Response,Box<dyn Error>> {
        Ok(SmtpTransport::relay(&mail.smtp_server)?.port(mail.smtp_port).credentials(mail.credentials).build().send(&Message::builder().from(sender.parse()?).to(recipient.parse()?).subject(subj).body(body.to_owned())?)?)
    }
}
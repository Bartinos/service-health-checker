use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;
use std::fmt::format;

pub struct MailController {
    from: String,
    to: String,
    subject: String,
    mail_key: String,
}

impl MailController {
    pub fn new() -> Self {
        let mail_from = std::env::var("MAIL_FROM").expect("MAIL_FROM must be set");
        let mail_to = std::env::var("MAIL_TO").expect("MAIL_TO must be set");
        let mail_key = std::env::var("MAIL_KEY").expect("MAIL_KEY must be set");

        MailController {
            from: mail_from 
                .parse()
                .unwrap(),
            to: mail_to 
                .parse()
                .unwrap(),
            subject: "Errors during service health checks".parse().unwrap(),
            mail_key,
        }
    }
    pub fn send_mail(&self, report: String) {
        let email = Message::builder()
            .from(self.from.parse().unwrap())
            .to(self.to.parse().unwrap())
            .subject(&self.subject)
            .header(ContentType::TEXT_PLAIN)
            .body(report)
            .unwrap();

        let creds = Credentials::new(
            "bartvanmoorsel7@gmail.com".to_owned(),
            self.mail_key.to_owned(),
        );

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {e:?}"),
        }
    }
}

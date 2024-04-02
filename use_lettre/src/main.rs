use lettre::{smtp::authentication::Credentials, SmtpClient, Transport};
use lettre_email::{EmailBuilder, Mailbox};

fn main() {
    let email = EmailBuilder::new()
        .from(Mailbox::new("1198816371@qq.com".to_string()))
        .to(Mailbox::new("1198816371@qq.com".to_string()))
        .subject("Test")
        .body("This is a test email.".to_string())
        .build()
        .unwrap();

    let creds = Credentials::new("1198816371".to_string(), "ltkldyvhtddafhbc".to_string());

    let mut mailer = SmtpClient::new_simple("smtp.qq.com")
        .unwrap()
        .credentials(creds)
        .transport();

    let result = mailer.send(email.into());

    if result.is_ok() {
        println!("email sent");
    } else {
        println!("not send email: {:?}", result);
    }
}

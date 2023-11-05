use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};

const EMAIL_FROM: &'static str = "noreply@muttr.com";

#[derive(Clone)]
pub struct Mailer(AsyncSmtpTransport<Tokio1Executor>);

impl Mailer {
    pub fn new(smtp_credentials: Credentials) -> Self {
        match AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.muttr.com") {
            Ok(builder) => {
                Mailer(builder.credentials(smtp_credentials).build())
            },
            Err(e) => panic!("Failed to init SMTP transport builder {:?}", e),
        }
    }

    pub async fn send_email(
        &self,
        to: &str,
        subject: &str,
        body: String
    ) -> Result<(), Box<dyn std::error::Error>> {
        let email = Message::builder()
            .from(EMAIL_FROM.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .body(body)?;
            
        self.0.send(email).await?;

        Ok(())
    }
}
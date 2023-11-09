extern crate samotop;

use crate::{config::Config, smtp_server::SmtpServer};
use lettre::transport::smtp::authentication::Credentials;
use samotop::{mail, smtp, server};
use secrecy::ExposeSecret;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;
use actix_web::{
    HttpServer,
    dev::Server,
    web::{get, Data},
};
use crate::{handlers::health_check, domain::mailer::Mailer};

pub struct App {
    port: u16,
    server: Server,
}

impl App {
    pub async fn build(config: Config) -> Result<Self, std::io::Error> {
        let address = format!(
            "{}:{}",
            config.app.host, config.app.port,
        );

        let smtp_credentials = Credentials::new(
            config.smtp.username,
            config.smtp.password.expose_secret().clone()
        );
        let smtp_port = config.smtp.port;
        let mailer = Mailer::new(smtp_credentials, config.smtp.port);

        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        
        let server = Self::run(listener, mailer)?;

        let _ = tokio::spawn(SmtpServer::start(smtp_port));

        Ok(Self{ port, server })
    }

    fn run(listener: TcpListener, mailer: Mailer) -> Result<Server, std::io::Error> {
        let mailer = Data::new(mailer);
        let server = HttpServer::new(move || {
            actix_web::App::new()
                .app_data(mailer.clone())
                .wrap(TracingLogger::default())
                .route("/health-check", get().to(health_check))
        })
        .listen(listener)?
        .run();
        Ok(server)
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}



use std::net::TcpListener;
use lettre::transport::smtp::authentication::Credentials;
use tracing_actix_web::TracingLogger;
use actix_web::{
    HttpServer, App,
    dev::Server,
    web::get,
};
use crate::{handlers::health_check, domain::mailer::Mailer};

pub fn run(listener: TcpListener, smtp_credentials: Credentials) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Mailer::new(smtp_credentials.clone()))
            .wrap(TracingLogger::default())
            .route("/health-check", get().to(health_check))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
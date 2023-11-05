use std::net::TcpListener;
use tracing_actix_web::TracingLogger;
use actix_web::{
    HttpServer, App,
    dev::Server,
    web::get,
};
use crate::handlers::health_check;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health-check", get().to(health_check))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
use std::net::TcpListener;
use secrecy::ExposeSecret;
use lettre::transport::smtp::authentication::Credentials;
use muttr_smtp_server::{
    startup::run,
    config::get_config,
    utils::telemetry::{create_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let default_filter_level = "info".to_string();
    let subscriber_name = "muttr_smtp_server".to_string();
    let subscriber = create_subscriber(
        subscriber_name, default_filter_level, std::io::stdout,
    );
    init_subscriber(subscriber);
    let config = get_config().expect("Failed to read config file");
    let smtp_credentials = Credentials::new(
        config.smtp.username.clone(), config.smtp.password.expose_secret().clone()
    );
    
    let address = format!("{}:{}", config.app.host, config.app.port);
    let listener = TcpListener::bind(address)
        .expect(&format!("Failed to bind to port {}", config.app.port));
    run(listener, smtp_credentials)?.await
}
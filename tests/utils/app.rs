use lettre::transport::smtp::authentication::Credentials;
use secrecy::ExposeSecret;
use once_cell::sync::Lazy;
use muttr_smtp_server::{
    startup::App,
    config::{Config, get_config},
    domain::mailer::Mailer,
    utils::telemetry::{create_subscriber, init_subscriber},
};
use super::client::Client;

static TRACING: Lazy<()> = Lazy::new(|| {
    let name = "test".to_string();
    let env_filter = "info".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = create_subscriber(name, env_filter, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = create_subscriber(name, env_filter, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub config: Config,
    pub address: String,
    pub client: Client,
    pub mailer: Mailer,
}

impl TestApp {
    pub async fn spawn() -> TestApp {
        std::env::set_var("APP_ENVIRONMENT", "test");
        Lazy::force(&TRACING);
        
        let config = {
            let mut c = get_config().expect("Failed to load test config file");
            c.app.port = 0;
            c
        };

        let app = App::build(config.clone())
            .await
            .expect("Failed to build app");
        let address = format!("http://127.0.0.1:{}", app.port());
        let _ = tokio::spawn(app.run_until_stopped());
    
        let smtp_credentials = Credentials::new(
            config.smtp.username.clone(), config.smtp.password.expose_secret().clone()
        );

        TestApp {
            config: config.clone(),
            address: address.clone(),
            client: Client::new(address),
            mailer: Mailer::new(smtp_credentials, config.smtp.port),
        }
    }
}

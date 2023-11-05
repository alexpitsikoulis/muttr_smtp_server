use muttr_smtp_server::{
    config::Config,
    utils::telemetry::{create_subscriber, init_subscriber},
};
use std::net::TcpListener;
use uuid::Uuid;
use once_cell::sync::Lazy;

const TEST_USER_EMAIL: &str = "testuser@youwish.com";
const TEST_USER_HANDLE: &str = "test.user";
const TEST_USER_PASSWORD: &str = "Testpassw0rd!"; 

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
}

pub async fn spawn_app() -> TestApp {
    std::env::set_var("APP_ENVIRONMENT", "test");
    Lazy::force(&TRACING);
    
    let config = muttr_smtp_server::config::get_config()
        .expect("Failed to load test config file");
    
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let server = muttr_smtp_server::startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        config,
        address,
    }
}
use muttr_smtp_server::{
    startup::App,
    config::get_config,
    utils::telemetry::{create_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = create_subscriber(
        "muttr_smtp_server".to_string(), "info".to_string(), std::io::stdout,
    );
    init_subscriber(subscriber);
    
    let config = get_config().expect("Failed to read config file");
    App::build(config).await?.run_until_stopped().await
}
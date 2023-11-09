use lettre::transport::smtp::authentication::Credentials;
use secrecy::{Secret, ExposeSecret};

#[derive(PartialEq)]
pub enum Env {
    Local,
    Production,
    Test,
}

impl Env {
    pub fn as_str(&self) -> &'static str {
        match self {
            Env::Local => "local",
            Env::Production => "production",
            Env::Test => "test",
        }
    }
}

impl From<String> for Env {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "local" => Self::Local,
            "production" => Self::Production,
            "test" => Self::Test,
            other => {
                tracing::warn!(
                    "{} is not a supported environment. \
                    Use either `local`, `production`, or `test`",
                    other
                );
                Self::Local
            },
        }
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct Config {
    pub app: AppConfig,
    pub smtp: SmtpConfig,
}

#[derive(serde::Deserialize, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct SmtpConfig {
    pub port: u16,
    pub username: String,
    pub password: Secret<String>,
}

impl SmtpConfig {
    pub fn get_credentials(&self) -> Credentials {
        Credentials::new(self.username.clone(), self.password.expose_secret().clone())
    }
}

pub fn get_config() -> Result<Config, config::ConfigError> {
    let base_path = std::env::current_dir()
        .expect("Failed to determine the current directory");
    let config_directory = base_path.join("config");
    let env: Env = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .into();
    let env_filename = format!("{}.yaml", env.as_str());
    
    let settings = config::Config::builder()
        .add_source(
            config::File::from(config_directory.join("base.yaml"))
        )
        .add_source(
            config::File::from(config_directory.join(&env_filename))
        )
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__")
        )
        .build()?;

    settings.try_deserialize::<Config>()
}
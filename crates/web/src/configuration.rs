use config::{Config, Environment, File};
use serde::Deserialize;

/// Application settings
#[derive(Deserialize, Debug)]
pub struct Settings {
    server: ServerSettings,
}

#[derive(Deserialize, Debug)]
pub struct ServerSettings {
    host: String,
    port: u16,
}

impl Settings {
    /// Returns the server address (host and port)
    pub fn address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    /// Load the settings from the configuration file (config/application.yaml)
    /// and environment variables.
    pub fn load() -> Result<Settings, config::ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config/application").required(false))
            .add_source(Environment::default().separator("_").ignore_empty(true))
            .build()?;
        s.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod settings {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_return_the_server_address() {
            let settings = Settings {
                server: ServerSettings {
                    host: String::from("127.0.0.1"),
                    port: 8080,
                },
            };

            assert_eq!("127.0.0.1:8080", settings.address());
        }
    }
}

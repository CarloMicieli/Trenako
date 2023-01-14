use config::{Config, Environment, File};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

/// Application settings
#[derive(Deserialize, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub server: ServerSettings,
}

impl Settings {
    /// Returns the server address (host and port)
    pub fn address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    /// Returns the number of actix workers
    pub fn workers(&self) -> usize {
        self.server.workers
    }

    /// Returns the postgres connection options
    pub fn pg_connection_options(&self) -> PgConnectOptions {
        let db_settings = &self.database;
        let ssl_mode = if db_settings.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .application_name("trenako")
            .host(&db_settings.host)
            .database(&db_settings.name)
            .username(&db_settings.username)
            .password(db_settings.password.expose_secret())
            .port(db_settings.port)
            .ssl_mode(ssl_mode)
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

/// It contains the server configuration
#[derive(Deserialize, Debug)]
pub struct ServerSettings {
    /// the server host name
    pub host: String,
    /// the server port number
    pub port: u16,
    /// the number of actix workers
    pub workers: usize,
}

/// It contains the database connection settings
#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    /// the username
    pub username: String,
    /// the password
    pub password: Secret<String>,
    /// the host name
    pub host: String,
    /// the port number
    pub port: u16,
    /// the database name
    pub name: String,
    /// the SSL mode for the connection
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn new(username: &str, password: &str, host: &str, port: u16, name: &str) -> DatabaseSettings {
        DatabaseSettings {
            username: username.to_owned(),
            password: Secret::new(password.to_owned()),
            host: host.to_owned(),
            port,
            name: name.to_owned(),
            require_ssl: false,
        }
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
                database: DatabaseSettings::new("postgres", "pa$$word", "database-host", 5432, "database-name"),
                server: ServerSettings {
                    host: String::from("127.0.0.1"),
                    port: 8080,
                    workers: 4,
                },
            };

            assert_eq!("127.0.0.1:8080", settings.address());
        }

        #[test]
        fn it_should_build_the_pg_connection_options() {
            let settings = Settings {
                database: DatabaseSettings::new("postgres", "pa$$word", "database-host", 5432, "database-name"),
                server: ServerSettings {
                    host: String::from("127.0.0.1"),
                    port: 8080,
                    workers: 4,
                },
            };

            let pg_connection_options = settings.pg_connection_options();
            assert_eq!(Some("database-name"), pg_connection_options.get_database());
        }
    }
}

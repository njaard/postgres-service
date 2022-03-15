//! Parse Postgres service configuration files

use ini::Properties;

fn build_from_section(section: &Properties) -> tokio_postgres::config::Config {
    let mut username: Option<String> = None;
    let mut password: Option<String> = None;

    let mut builder = tokio_postgres::config::Config::new();
    let mut options = String::new();

    for (k, v) in section.iter() {
        match k {
            "host" => {
                builder.host(v);
            }
            "hostaddr" => {
                builder.host(v);
            }
            "port" => {
                builder.port(v.parse().unwrap());
            }
            "dbname" => {
                builder.dbname(v);
            }
            "user" => username = Some(v.to_owned()),
            "password" => password = Some(v.to_owned()),
            _ => options += &format!("{}={} ", k, v),
        }
    }

    if !options.is_empty() {
        builder.options(&options);
    }

    if let Some(username) = username {
        builder.user(&username);
    }
    if let Some(password) = password {
        builder.password(&password);
    }

    builder
}

/// Load connection parameters for the named Postgresql service
pub fn load_connect_params(service_name: &str) -> Option<postgres::config::Config> {
    tokio::load_connect_params(service_name).map(Into::into)
}

/// For use with tokio-postgres
pub mod tokio {
    use crate::build_from_section;
    use ini::Ini;
    /// Load connection parameters for the named Postgresql service
    pub fn load_connect_params(service_name: &str) -> Option<tokio_postgres::config::Config> {
        if let Ok(home) = std::env::var("HOME") {
            if let Ok(ini) = Ini::load_from_file(home + "/" + ".pg_service.conf") {
                if let Some(section) = ini.section(Some(service_name.clone())) {
                    return Some(build_from_section(section));
                }
            }
        }

        let confdir = std::env::var("PGSYSCONFDIR").unwrap_or("/etc/postgresql-common".into());

        if let Ok(ini) = Ini::load_from_file(confdir + "/" + "pg_service.conf") {
            if let Some(section) = ini.section(Some(service_name)) {
                return Some(build_from_section(section));
            }
        }

        None
    }
}

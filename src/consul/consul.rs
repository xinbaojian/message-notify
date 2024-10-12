use rocket::log::private::{log, Level};
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ConsulConfig {
    port: u16,
    consul_address: String,
    service_name: String,
    ip_address: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct ServiceRegistration {
    #[serde(rename = "ID")]
    id: String,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Address")]
    address: String,

    #[serde(rename = "Port")]
    port: u16,

    #[serde(rename = "Check")]
    check: ServiceCheck,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServiceCheck {
    #[serde(rename = "HTTP")]
    http: String,

    #[serde(rename = "Interval")]
    interval: String,

    #[serde(rename = "Timeout")]
    timeout: String,
}

pub async fn register_service(config: &ConsulConfig) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let service = ServiceRegistration {
        id: format!("{}-{}", config.service_name, config.port),
        name: config.service_name.clone(),
        address: config.ip_address.clone(),
        port: config.port,
        check: ServiceCheck {
            http: format!("http://{}:{}/health", config.ip_address, config.port),
            interval: "10s".into(),
            timeout: "5s".into(),
        },
    };

    let res = client
        .put(format!(
            "{}/v1/agent/service/register",
            config.consul_address
        ))
        .json(&service)
        .send()
        .await?;
    if res.status().is_success() {
        log!(Level::Info, "Service registered successfully");
    } else {
        log!(Level::Error, "Failed to register service: {}", res.text().await?);
    }
    Ok(())
}

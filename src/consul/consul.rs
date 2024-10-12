use rocket::log::private::{log, Level};
use rocket::serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ConsulConfig {
    port: u16,
    consul_address: String,
    service_name: String,
    ip_address: String,
    ip_port: u16,
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
    // 从环境变量中获取变量值
    let address = env::var("CONSUL_ADDRESS").unwrap_or_else(|_| config.consul_address.clone());
    let service_name =
        env::var("CONSUL_SERVICE_NAME").unwrap_or_else(|_| config.service_name.clone());
    let ip_address = env::var("CONSUL_IP_ADDRESS").unwrap_or_else(|_| config.ip_address.clone());
    let ip_port = env::var("CONSUL_IP_PORT").unwrap_or_else(|_| config.ip_port.to_string());
    let client = reqwest::Client::new();
    let service = ServiceRegistration {
        id: format!("{}-{}", service_name, config.port),
        name: address,
        address: ip_address.clone(),
        port: config.port,
        check: ServiceCheck {
            http: format!("http://{}:{}/health", ip_address, ip_port),
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
        log!(
            Level::Error,
            "Failed to register service: {}",
            res.text().await?
        );
    }
    Ok(())
}

use rdkafka::config::ClientConfig;
use rdkafka::producer::FutureProducer;
use std::env;
use std::error::Error;

pub struct KafkaClientConfig {
    broker_endpoint: String,
    cluster_api_key: String,
    cluster_api_secret: String,
}

impl KafkaClientConfig {
    pub fn new(
        broker_endpoint: &str,
        cluster_api_key: &str,
        cluster_api_secret: &str,
    ) -> Self {
        Self {
            broker_endpoint: broker_endpoint.to_string(),
            cluster_api_key: cluster_api_key.to_string(),
            cluster_api_secret: cluster_api_secret.to_string(),
        }
    }

    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        let broker_endpoint = env::var("KAFKA_BROKER_ENDPOINT")?;
        let cluster_api_key = env::var("KAFKA_CLUSTER_API_KEY")?;
        let cluster_api_secret = env::var("KAFKA_CLUSTER_API_SECRET")?;

        Ok(Self::new(
            &broker_endpoint,
            &cluster_api_key,
            &cluster_api_secret,
        ))
    }
}

pub struct KafkaClient {
    config: KafkaClientConfig,
    producer: Option<FutureProducer>,
}

impl KafkaClient {
    pub fn new(config: KafkaClientConfig) -> Self {
        Self {
            config,
            producer: None,
        }
    }

    pub fn init_connection(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        openssl_probe::init_ssl_cert_env_vars();

        self.producer = Some(
            ClientConfig::new()
                .set("bootstrap.servers", self.config.broker_endpoint.clone())
                .set("security.protocol", "SASL_SSL")
                .set("sasl.mechanisms", "PLAIN")
                .set("sasl.username", self.config.cluster_api_key.clone())
                .set("sasl.password", self.config.cluster_api_secret.clone())
                .create::<FutureProducer>()?,
        );

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_connection() {
      let mut kafka_client =
      KafkaClient::new(KafkaClientConfig::from_env().expect("Failed to init KafkaClientConfig"));

      kafka_client.init_connection().expect("Failed initializing connection");
    }
}
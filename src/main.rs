mod kafka_client;
use kafka_client::{KafkaClient, KafkaClientConfig};
use lambda_http::{
    lambda_runtime::Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut kafka_client =
        KafkaClient::new(KafkaClientConfig::from_env().expect("Failed to init KafkaClientConfig"));

    kafka_client.init_connection().expect("Failed to initialize connection to Kafka cluster");

    Ok(())
}

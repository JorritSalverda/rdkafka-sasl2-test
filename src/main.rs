mod kafka_client;
use kafka_client::{KafkaClient, KafkaClientConfig};

fn main() {
    let mut kafka_client =
        KafkaClient::new(KafkaClientConfig::from_env().expect("Failed to init KafkaClientConfig"));

    kafka_client.init_connection().expect("Failed to initialize connection to Kafka cluster");
}

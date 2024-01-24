use std::time::Duration;

use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};

pub async fn send_message(bootstrap_servers: &str, topic: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", bootstrap_servers)
        .create()?;

    let record = FutureRecord::to(topic)
        .payload(message)
        .key("key");

    match producer.send(record, Duration::from_secs(0)).await {
        Ok(_) => println!("Message sent successfully"),
        Err((e, _)) => println!("Failed to send message: {}", e),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use testcontainers::{clients::Cli, core::WaitFor, Image, RunnableImage};

    #[derive(Default)]
    struct HelloWorld {
        volumes: BTreeMap<String, String>,
        env_vars: BTreeMap<String, String>,
    }

    impl Image for HelloWorld {
        type Args = ();

        fn name(&self) -> String {
            "confluentinc/cp-kafka".to_owned()
        }

        fn tag(&self) -> String {
            "latest".to_owned()
        }

        fn ready_conditions(&self) -> Vec<WaitFor> {
            vec![WaitFor::message_on_stdout("Hello from Docker!")]
        }

        fn env_vars(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
            Box::new(self.env_vars.iter())
        }

        fn volumes(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
            Box::new(self.volumes.iter())
        }
    }

    use crate::send_message;
    #[tokio::test]
    async fn test_kafka_producer() {
        //let docker = clients::Cli::default();
        let docker = Cli::default();
        let kafka_node = docker.run(RunnableImage::from(HelloWorld::default()));
    
        let bootstrap_servers = format!("localhost:{}", kafka_node.get_host_port_ipv4(9093));
        let topic = "test-topic";
    
        // ここでProducerを使ってメッセージを送信
        send_message(&bootstrap_servers, topic, "Hello Kafka").await.expect("Failed to send message");
    
        // ここで送信されたメッセージを検証
        // 実際のテストではConsumerを使用して送信されたメッセージを検証する必要があります
    }
}
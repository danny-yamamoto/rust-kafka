# rust-kafka
About Rust and Kafka.

```bash
cd dev/rust-kafka
docker compose up -d
docker ps -a
```

```bash
yamamotodaisuke@PS2-10160074 rust-kafka % docker ps -a
CONTAINER ID   IMAGE                                               COMMAND                   CREATED         STATUS          PORTS                                         NAMES
84b0e73c2763   confluentinc/cp-kafka:latest                        "/etc/confluent/dock…"   2 minutes ago   Up 14 seconds   9092/tcp, 0.0.0.0:29092->29092/tcp            rust-kafka-kafka-1
9c8fcdcf1e26   confluentinc/cp-zookeeper:latest                    "/etc/confluent/dock…"   2 minutes ago   Up 2 minutes    2888/tcp, 3888/tcp, 0.0.0.0:22181->2181/tcp   rust-kafka-zookeeper-1
fd37ca4bfd16   mcr.microsoft.com/devcontainers/rust:1-1-bullseye   "/bin/sh -c 'echo Co…"   5 minutes ago   Up 5 minutes                                                  elastic_herschel
yamamotodaisuke@PS2-10160074 rust-kafka %
```
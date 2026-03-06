# KafkaProxy
A really slim, stateless. probably only useful for this project kafka relay

## How does it work
It connects to both an "upstream" kafka broker and a "downstream" kafka broker. and forwards the messages from upstream -> downstream. Simples.

It supports:
- Upstream PLAIN SASL Authentication
- Nothing else.

## Running
## Environment Variables
| Variable | Example | Description
| --- | --- | --- |
| UPSTREAM_KAFKA_HOST | | Endpoint (including port) of the upstream |
| DOWNSTREAM_KAFKA_HOST | | Endpoint (including port) of the downstream |
| KAFKA_TOPIC | | The topic to listen on and relay to |
| KAFKA_USERNAME | | The username to authenticate with upstream with |
| KAFKA_PASSWORD | | The password to authenticate with upstream with |
| KAFKA_GROUP | | The group to use with upstream |

This project like everything else is designed to run within nix.
1. `nix develop`
2. Make sure your kafka broker is running
2. `cargo run --bin kafkaproxy`
3. Success!
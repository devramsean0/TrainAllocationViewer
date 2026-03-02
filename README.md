# Train Application Viewer
I promise this diagram is not AI sob
```mermaid
flowchart LR
    TrainConsist[Train Consist & Allocations Feed] -->|Kafka| KafkaProxy1
    KafkaProxy1[Kafka Proxy] -->|Kafka| API
    TrainConsistArchive[Train Consist & Allocations Archive] -->|S3| API
    CORPUSData[Corpus Location Data] -->|S3| API

    API -->|GraphQL|AndroidApp[Android App]
    API -->|GraphQL|Web[Website]
    
```

## API/Rust Environment Setup
We use Nix!
1. `nix develop`
2. `Copy and complete .env.example`

# Notes:
## Build updated kotlin sdk version:
```sh
cd packages/sdk
cargo build --release
cargo run --bin uniffi-bindgen generate --library target/release/libsdk.so --language kotlin --out-dir ../../target
```
use log::info;
use rdkafka::{consumer::Consumer, Message};
use tokio::signal;

mod kafka;
mod payload;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let kafka = kafka::create_consumer()?;
    kafka.subscribe(&["prod-1033-Passenger-Train-Allocation-and-Consist-1_0"])?;

    info!("Listening for messages…");

    loop {
        tokio::select! {
            msg = kafka.recv() => {
                match msg {
                    Err(e) => eprintln!("Kafka error: {e}"),
                    Ok(m) => {
                        if let Some(payload) = m.payload() {
                            payload::handle_payload(payload)?;
                        }
                    }
                }
            }
            _ = signal::ctrl_c() => {
                println!("Shutting down");
                break;
            }
        }
    }
    Ok(())
}

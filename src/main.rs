use log::info;
use rdkafka::{consumer::Consumer, Message};
use tokio::signal;

use crate::db::schema::{Allocation, ResourceGroup, Vehicle};

mod db;
mod kafka;
mod payload;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let pool = sqlx::sqlite::SqlitePool::connect("sqlite:data.db").await?;
    sqlx::migrate!().run(&pool).await?;

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
                            let parsed = payload::handle_payload(payload)?;
                            if parsed.allocation.is_some() {
                                for allocation in parsed.allocation.unwrap() {
                                    let resource_group = db::resource_group::ResourceGroup::insert(&pool, ResourceGroup {
                                        id: allocation.resource_group.resource_group_id.clone(),
                                        fleet: allocation.resource_group.fleet_id
                                    }).await?;

                                    db::allocation::Allocation::insert(&pool, Allocation {
                                        id: None,
                                        origin_datetime: allocation.train_origin_date_time,
                                        origin_location: allocation.train_origin_location.location_primary_code,
                                        date: allocation.diagram_date,
                                        dest_location: allocation.train_dest_location.location_primary_code,
                                        dest_datetime: allocation.train_dest_date_time,
                                        allocation_origin_datetime: allocation.allocation_origin_date_time,
                                        allocation_origin_location: allocation.allocation_origin_location.location_primary_code,
                                        allocation_dest_datetime: allocation.allocation_destination_date_time,
                                        allocation_dest_location: allocation.allocation_destination_location.location_primary_code,
                                        resource_group_id: resource_group.id.clone(),
                                    }).await?;

                                    for vehicle in allocation.resource_group.vehicle {
                                        db::vehicle::Vehicle::insert(&pool, Vehicle {
                                            id: Some(vehicle.vehicle_id),
                                            decor: vehicle.decor,
                                            livery: vehicle.livery,
                                            specific_type: vehicle.specific_type,
                                            vehicle_type: vehicle.type_of_vehicle,
                                            resource_group_id: resource_group.id.clone(),
                                        }).await?;
                                    }
                                }
                            }
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

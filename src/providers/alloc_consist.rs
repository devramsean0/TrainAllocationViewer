use rdkafka::message::ToBytes;

use crate::db::schema::{Allocation, ResourceGroup, Vehicle};

pub fn callback(
    msg: Vec<u8>,
    pool: &'static sqlx::SqlitePool,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + 'static>> {
    Box::pin(async move {
        let parsed = crate::payload::handle_payload(msg.to_bytes())?;
        if parsed.allocation.is_some() {
            for allocation in parsed.allocation.unwrap() {
                let resource_group = crate::db::resource_group::ResourceGroup::insert(
                    &pool,
                    ResourceGroup {
                        id: allocation.resource_group.resource_group_id.clone(),
                        fleet: allocation.resource_group.fleet_id,
                    },
                )
                .await?;

                crate::db::allocation::Allocation::insert(
                    &pool,
                    Allocation {
                        id: None,
                        origin_datetime: allocation.train_origin_date_time,
                        origin_location: allocation.train_origin_location.location_primary_code,
                        date: allocation.diagram_date,
                        dest_location: allocation.train_dest_location.location_primary_code,
                        dest_datetime: allocation.train_dest_date_time,
                        allocation_origin_datetime: allocation.allocation_origin_date_time,
                        allocation_origin_location: allocation
                            .allocation_origin_location
                            .location_primary_code,
                        allocation_dest_datetime: allocation.allocation_destination_date_time,
                        allocation_dest_location: allocation
                            .allocation_destination_location
                            .location_primary_code,
                        resource_group_id: resource_group.id.clone(),
                    },
                )
                .await?;

                for vehicle in allocation.resource_group.vehicle {
                    crate::db::vehicle::Vehicle::insert(
                        &pool,
                        Vehicle {
                            id: Some(vehicle.vehicle_id),
                            decor: vehicle.decor,
                            livery: vehicle.livery,
                            specific_type: vehicle.specific_type,
                            vehicle_type: vehicle.type_of_vehicle,
                            resource_group_id: resource_group.id.clone(),
                        },
                    )
                    .await?;
                }
            }
        }
        Ok(())
    })
}

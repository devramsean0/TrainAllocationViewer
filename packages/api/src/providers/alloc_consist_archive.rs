use flate2::read::GzDecoder;
use log::{debug, info};
use rdkafka::message::ToBytes;
use tokio::spawn;

use std::io::Read;

use crate::{
    db::schema::{AllocArchiveLog, Allocation, ResourceGroup, Vehicle},
    sources::s3::S3Client,
};
use futures::stream::TryStreamExt;
use std::fs;

async fn decide_on_download(file: String) -> anyhow::Result<bool> {
    if !fs::exists(format!("cache/{file}"))? {
        info!("Downloading {file} because cache not found");
        return Ok(true);
    } else {
        let client = S3Client::new()?;
        let s3_file_head = client.head(file.clone()).await?;
        let s3_md5 = s3_file_head.etag.unwrap().replace("\"", "");
        let md5 = crate::utils::md5::md5_of_file(format!("cache/{file}").as_str()).unwrap();

        debug!("S3: {}, local: {md5}", s3_md5);

        if s3_md5.contains("-") {
            debug!("Multipart Upload detected, falling back to content length");
            if s3_file_head.content_length
                == Some(fs::read(format!("cache/{file}")).unwrap().len() as u64)
            {
                info!("Skipping Download because content length matches");
                return Ok(false);
            } else {
                info!("Downloading because content length doesn't match");
                return Ok(true);
            }
        }

        if s3_md5 == md5 {
            info!("Skipping download because cache matches");
            return Ok(false);
        }
        info!("Downloading because cache doesn't match");
        Ok(true)
    }
}
pub async fn download_archive(pool: &'static sqlx::PgPool) -> anyhow::Result<()> {
    spawn(async move {
        let client = S3Client::new().unwrap();
        let files = client.listv2("passenger-consist-").await.unwrap();

        let mut message_count: i128 = 0;
        for file in files.contents {
            let file_key: String = file.key.clone();
            let db = AllocArchiveLog::get_by_filename(pool, file_key.clone()).await;
            match db {
                Ok(_) => {
                    info!("Skipping {file_key} because already in DB");
                }
                Err(_) => {
                    if decide_on_download(file.key.clone()).await.unwrap() {
                        let s3_file_body = client.get(file.key.clone()).await.unwrap();

                        let mut body_stream = s3_file_body.body;
                        let mut bytes = Vec::new();
                        while let Some(chunk) = body_stream.try_next().await.unwrap() {
                            bytes.extend_from_slice(&chunk);
                        }
                        fs::create_dir_all("cache").unwrap();
                        fs::write(format!("cache/{}", file.key), bytes.clone()).unwrap();
                    }

                    let file = fs::read(format!("cache/{}", file.key)).unwrap();
                    let mut decompressor = GzDecoder::new(file.to_bytes());
                    let mut xml_string = String::new();
                    decompressor.read_to_string(&mut xml_string).unwrap();

                    info!("Decompressed {file_key}");

                    let messages = parse_xml_messages(&xml_string);

                    for xml_content in messages {
                        let parsed = crate::payload::handle_payload(xml_content.as_bytes())
                            .unwrap_or_default();
                        if parsed.allocation.is_some() {
                            for allocation in parsed.allocation.unwrap() {
                                let resource_group =
                                    crate::db::resource_group::ResourceGroup::insert(
                                        &pool,
                                        ResourceGroup {
                                            id: allocation.resource_group.resource_group_id.clone(),
                                            fleet: allocation.resource_group.fleet_id,
                                            resource_type: Some(
                                                allocation
                                                    .resource_group
                                                    .type_of_resource
                                                    .to_string(),
                                            ),
                                            status: Some(
                                                allocation.resource_group.resource_group_status,
                                            ),
                                            end_of_day_miles: Some(
                                                allocation.resource_group.end_of_day_miles,
                                            ),
                                        },
                                    )
                                    .await
                                    .unwrap();

                                crate::db::allocation::Allocation::insert(
                                    &pool,
                                    Allocation {
                                        id: None,
                                        origin_datetime: allocation.train_origin_date_time,
                                        origin_location: allocation
                                            .train_origin_location
                                            .location_primary_code,
                                        origin_country_code_iso: Some(
                                            allocation.train_origin_location.country_code_iso,
                                        ),
                                        origin_subsidiary_information_code: Some(
                                            allocation
                                                .train_origin_location
                                                .location_subsidiary_identification
                                                .location_sibsidiary_code,
                                        ),
                                        origin_subsidiary_information_company: Some(
                                            allocation
                                                .train_origin_location
                                                .location_subsidiary_identification
                                                .allocation_company,
                                        ),
                                        date: allocation.diagram_date,
                                        dest_location: allocation
                                            .train_dest_location
                                            .location_primary_code,
                                        dest_country_code_iso: Some(
                                            allocation.train_dest_location.country_code_iso,
                                        ),
                                        dest_subsidiary_information_code: Some(
                                            allocation
                                                .train_dest_location
                                                .location_subsidiary_identification
                                                .location_sibsidiary_code,
                                        ),
                                        dest_subsidiary_information_company: Some(
                                            allocation
                                                .train_dest_location
                                                .location_subsidiary_identification
                                                .allocation_company,
                                        ),
                                        dest_datetime: allocation.train_dest_date_time,
                                        allocation_origin_datetime: allocation
                                            .allocation_origin_date_time,
                                        allocation_origin_location: allocation
                                            .allocation_origin_location
                                            .location_primary_code,
                                        allocation_origin_country_code_iso: Some(
                                            allocation.allocation_origin_location.country_code_iso,
                                        ),
                                        allocation_origin_subsidiary_information_code: Some(
                                            allocation
                                                .allocation_origin_location
                                                .location_subsidiary_identification
                                                .location_sibsidiary_code,
                                        ),
                                        allocation_origin_subsidiary_information_company: Some(
                                            allocation
                                                .allocation_origin_location
                                                .location_subsidiary_identification
                                                .allocation_company,
                                        ),
                                        allocation_dest_datetime: allocation
                                            .allocation_destination_date_time,
                                        allocation_dest_location: allocation
                                            .allocation_destination_location
                                            .location_primary_code,
                                        allocation_dest_country_code_iso: Some(
                                            allocation
                                                .allocation_destination_location
                                                .country_code_iso,
                                        ),
                                        allocation_dest_subsidiary_information_code: Some(
                                            allocation
                                                .allocation_destination_location
                                                .location_subsidiary_identification
                                                .location_sibsidiary_code,
                                        ),
                                        allocation_dest_subsidiary_information_company: Some(
                                            allocation
                                                .allocation_destination_location
                                                .location_subsidiary_identification
                                                .allocation_company,
                                        ),
                                        resource_group_id: resource_group.id.clone(),
                                        sequence_number: Some(
                                            allocation.allocation_sequence_number,
                                        ),
                                        resource_group_position: Some(
                                            allocation.resource_group_position,
                                        ),
                                        diagram_no: allocation.diagram_no,
                                        origin_miles: Some(allocation.allocation_origin_miles),
                                        destination_miles: Some(
                                            allocation.allocation_destination_miles,
                                        ),
                                        reversed: Some(allocation.reversed),
                                    },
                                )
                                .await
                                .unwrap();

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
                                            resource_position: Some(vehicle.resource_position),
                                            planned_resource_group: vehicle.planned_resource_group,
                                            length_value: Some(vehicle.length.value),
                                            length_measure: Some(vehicle.length.measure),
                                            weight: Some(vehicle.weight),
                                            special_characteristics: vehicle
                                                .special_characteristics,
                                            seat_count: vehicle.number_of_seats,
                                            cab_count: vehicle.cabs,
                                            date_entered_service: Some(
                                                vehicle.date_entered_service,
                                            ),
                                            date_registered: Some(vehicle.date_registered),
                                            category: Some(vehicle.registered_category),
                                            brake_type: Some(vehicle.train_brake_type),
                                            max_speed: Some(vehicle.maximum_speed),
                                        },
                                    )
                                    .await
                                    .unwrap();
                                }
                            }
                            message_count += 1;
                            if message_count % 1000 == 0 {
                                info!("Processed 1000 messages ({message_count})");
                            }
                        }
                    }
                    info!("DB insert completed for {file_key}");
                    AllocArchiveLog::insert(
                        pool,
                        AllocArchiveLog {
                            id: None,
                            file_name: file_key,
                        },
                    )
                    .await
                    .unwrap();
                }
            }
        }
    });
    Ok(())
}

/// Parses XML messages from a string that may contain multiple messages.
/// Each message is prefixed with a message ID followed by '$', and the XML content
/// can span multiple lines. Messages start with '<?xml' or '<PassengerTrainConsistMessage'.
/// Function contributed by AI, because I am an idiot :3
fn parse_xml_messages(content: &str) -> Vec<String> {
    let mut messages = Vec::new();
    let mut current_message = String::new();
    let mut in_message = false;

    for line in content.lines() {
        if let Some(dollar_pos) = line.find('$') {
            let after_dollar = &line[dollar_pos + 1..];
            if after_dollar.trim_start().starts_with("<?xml")
                || after_dollar
                    .trim_start()
                    .starts_with("<PassengerTrainConsistMessage")
            {
                if in_message && !current_message.trim().is_empty() {
                    messages.push(current_message.trim().to_string());
                }
                current_message = after_dollar.to_string();
                in_message = true;
                continue;
            }
        }

        if line.trim_start().starts_with("<?xml")
            || line
                .trim_start()
                .starts_with("<PassengerTrainConsistMessage")
        {
            if in_message && !current_message.trim().is_empty() {
                messages.push(current_message.trim().to_string());
            }
            current_message = line.to_string();
            in_message = true;
            continue;
        }

        if in_message {
            current_message.push('\n');
            current_message.push_str(line);
        }
    }
    if in_message && !current_message.trim().is_empty() {
        messages.push(current_message.trim().to_string());
    }

    messages
}

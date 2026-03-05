use crate::{
    db::{bplan_log::BplanLog, reference_codes::ReferenceCode},
    sources::s3::S3Client,
};
use csv::ReaderBuilder;
use flate2::read::GzDecoder;
use log::{debug, info};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use zip::ZipArchive;

use std::{
    fmt,
    fs::File,
    io::{Cursor, Read},
};

use futures::stream::TryStreamExt;
use std::fs;

async fn decide_on_download() -> anyhow::Result<bool> {
    if !fs::exists("cache/BPLAN Geography Dataset.zip")? {
        info!("Downloading because cache not found");
        return Ok(true);
    } else {
        if std::env::var("ENABLE_BPLAN_UPDATE")? == "true" {
            info!("Downloading because environment variable set");
            let client = S3Client::new()?;
            let s3_file_head = client
                .head("BPLAN Geography Dataset.zip".to_string())
                .await?;
            let s3_md5 = s3_file_head.etag.unwrap().replace("\"", "");
            let md5 = crate::utils::md5::md5_of_file("cache/BPLAN Geography Dataset.zip").unwrap();

            debug!("S3: {}, local: {md5}", s3_md5);
            if s3_md5 == md5 {
                info!("Skipping download because cache matches");
                return Ok(false);
            }
            info!("Downloading because cache doesn't match");
        } else {
            return Ok(false);
        }
        Ok(true)
    }
}
pub async fn update_bplan(pool: &Pool<Postgres>) -> anyhow::Result<()> {
    if decide_on_download().await? {
        let client = S3Client::new()?;
        let s3_file_body = client
            .get("BPLAN Geography Dataset.zip".to_string())
            .await?;

        let mut body_stream = s3_file_body.body;
        let mut bytes = Vec::new();
        while let Some(chunk) = body_stream.try_next().await? {
            bytes.extend_from_slice(&chunk);
        }
        fs::create_dir_all("cache")?;
        fs::write("cache/BPLAN Geography Dataset.zip", bytes.clone())?;
    }

    let file = File::open("cache/BPLAN Geography Dataset.zip")?;
    let mut zip = ZipArchive::new(file)?;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        debug!("Discovered File Name: {}", entry.name());
        // For text-based files, try reading them as UTF-8
        if entry.name().ends_with(".txt.gz") {
            let mut file_bytes = Vec::new();
            entry.read_to_end(&mut file_bytes)?;
            let mut decompressor = GzDecoder::new(&file_bytes[..]);
            let mut data_string = String::new();
            decompressor.read_to_string(&mut data_string).unwrap();
            info!("Blan {} length: {}", entry.name(), data_string.len());

            let mut continue_parsing = true;
            let mut reference_records: Vec<ReferenceCode> = vec![];
            for line in data_string.split("\n") {
                if continue_parsing {
                    let mut parts = line.split('\t');
                    let record_type = parts.next().unwrap();
                    let rest = parts.collect::<Vec<_>>().join("\t");

                    match record_type {
                        "PIF" => {
                            let mut rdr = ReaderBuilder::new()
                                .delimiter(b'\t')
                                .has_headers(false)
                                .from_reader(Cursor::new(rest));

                            let record: Header = rdr.deserialize().next().unwrap()?;
                            debug!("PIF: {:#?}", record);
                            match BplanLog::get_by_creation_date(pool, record.creation_date.clone())
                                .await
                            {
                                Ok(_log) => {
                                    continue_parsing = false;
                                }
                                Err(_) => {
                                    BplanLog::insert(
                                        pool,
                                        BplanLog {
                                            id: None,
                                            toc_id: record.toc_id,
                                            file_version: record.file_version,
                                            source_system: record.source_system,
                                            timetable_start_date: record.timetable_start_date,
                                            timetable_end_date: record.timetable_end_date,
                                            cycle_type: record.cycle_type.to_string(),
                                            cycle_stage: record.cycle_stage,
                                            creation_date: record.creation_date,
                                            sequence_number: record.sequence_number,
                                        },
                                    )
                                    .await?;
                                }
                            };
                        }
                        "REF" => {
                            let mut rdr = ReaderBuilder::new()
                                .delimiter(b'\t')
                                .has_headers(false)
                                .from_reader(Cursor::new(rest));

                            let record: Reference = rdr.deserialize().next().unwrap()?;
                            reference_records.push(ReferenceCode {
                                id: None,
                                action_code: record.action_code,
                                code_type: record.code_type.to_string(),
                                code: record.code,
                                description: record.description,
                            });
                        }
                        _ => {} // Do nothing because we don't care
                    }
                }
            }
            ReferenceCode::insert_bulk(pool, &reference_records).await?
        } else {
            debug!("Skipping because not gzipped")
        }
    }
    Ok(())
}

#[derive(Deserialize, Debug)]
struct Header {
    file_version: String,
    source_system: String,
    toc_id: String,
    timetable_start_date: String,
    timetable_end_date: String,
    cycle_type: HeaderCycleTypeEnum, // Change to Enum
    cycle_stage: String,
    creation_date: String,
    sequence_number: i32,
}

#[derive(Deserialize, Debug, Default)]
pub enum HeaderCycleTypeEnum {
    #[default]
    Unknown,
    #[serde(rename = "S")]
    Supplimental,
    #[serde(rename = "I")]
    Iterative,
}

impl fmt::Display for HeaderCycleTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Debug, Default)]
struct Reference {
    action_code: String,
    code_type: ReferenceCodeTypeEnum,
    code: Option<String>,
    description: String,
}

#[derive(Deserialize, Debug, Default)]
pub enum ReferenceCodeTypeEnum {
    #[default]
    Unknown,
    #[serde(rename = "ACC")]
    Accomodation,
    #[serde(rename = "ACT")]
    Activities,
    #[serde(rename = "BHX")]
    BankHolidayExcepted,
    #[serde(rename = "BRA")]
    ServiceBrands,
    #[serde(rename = "BUS")]
    BusinessSector,
    #[serde(rename = "CAT")]
    Catering,
    #[serde(rename = "OPC")]
    OperatingCharacteristic,
    #[serde(rename = "PWR")]
    PowerType,
    #[serde(rename = "RES")]
    Reservation,
    #[serde(rename = "SER")]
    Service,
    #[serde(rename = "SLE")]
    Sleeper,
    #[serde(rename = "TCL")]
    TrainClass,
    #[serde(rename = "TCT")]
    TrainCategory,
    #[serde(rename = "TOC")]
    TrainOperator,
    #[serde(rename = "TRS")]
    TrainStatus,
    #[serde(rename = "TST")]
    TrainPublicationStatus,
    #[serde(rename = "REF")]
    ReferenceCodeTypes,
    #[serde(rename = "ZNE")]
    NetworkRailZone,
}

impl fmt::Display for ReferenceCodeTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

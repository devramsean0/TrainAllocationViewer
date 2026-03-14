use fixed_width::from_bytes;
use fixed_width_derive::FixedWidth;
use flate2::read::GzDecoder;
use log::{debug, error, info};
use rdkafka::message::ToBytes;
use serde::Deserialize;
use sqlx::{Pool, Postgres};

use std::{collections::HashMap, io::Read};

use crate::{
    db::schema::{CifScheduleLog, Location, Schedule, ScheduleLocation},
    sources::nr_static::NRStatic,
    utils,
};
use std::fs;

async fn decide_on_download() -> anyhow::Result<bool> {
    if !fs::exists("cache/schedule-all-toc-weekly-cif.gz")? {
        info!("Downloading because cache not found");
        return Ok(true);
    } else {
        if std::env::var("ENABLE_SCHEDULE_UPDATE")? == "true" {
            info!("Downloading because environment variable set");
            let client = NRStatic::new()?;
            let etag = client.fetch_etag("https://publicdatafeeds.networkrail.co.uk/ntrod/CifFileAuthenticate?type=CIF_ALL_FULL_DAILY&day=toc-full.CIF.gz".to_string()).await?;
            let local = utils::md5::md5_of_file("cache/schedule-all-toc-weekly-cif.gz").unwrap();
            debug!("NWR: {etag}, local: {local}");
            if etag == local {
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
pub async fn update_schedule(pool: &Pool<Postgres>) -> anyhow::Result<()> {
    if decide_on_download().await? {
        let client = NRStatic::new()?;
        let bytes = client.fetch_full("https://publicdatafeeds.networkrail.co.uk/ntrod/CifFileAuthenticate?type=CIF_ALL_FULL_DAILY&day=toc-full.CIF.gz".to_string()).await?;

        fs::create_dir_all("cache")?;
        fs::write("cache/schedule-all-toc-weekly-cif.gz", bytes.clone())?;
    }

    let file = fs::read("cache/schedule-all-toc-weekly-cif.gz")?;
    let mut decompressor = GzDecoder::new(file.to_bytes());
    let mut schedule_string = String::new();
    decompressor.read_to_string(&mut schedule_string).unwrap();

    let mut reference_locations: HashMap<String, String> = HashMap::new();
    for loc in Location::get_locations_where_has_tiploc(pool).await? {
        reference_locations.insert(loc.tiploc.unwrap(), loc.uic.unwrap());
    }

    info!("Prep Work Done, Processing now");

    let mut locations: Vec<LocationTypes> = vec![];
    let mut schedule: Option<StructuredSchedule> = None;

    for line in schedule_string.split("\n") {
        match &line[0..2] {
            "HD" => {
                let data = from_bytes::<CIFHeader>(line.as_bytes()).ok();
                if data.is_some() {
                    let data = data.unwrap();
                    CifScheduleLog::insert(
                        pool,
                        CifScheduleLog {
                            id: None,
                            mainframe_identity: data.mainframe_identity,
                            extract_date: data.extract_date,
                            extract_time: data.extract_time,
                            file_reference: data.current_file_reference,
                            version: data.version,
                        },
                    )
                    .await?;
                }
            }
            "BS" => match from_bytes::<CIFBasicSchedule>(line.as_bytes()).ok() {
                Some(data) => {
                    schedule = Some(StructuredSchedule {
                        uid: data.train_uid,
                        identity: data.train_identity,
                        headcode: data.train_headcode,
                        start_date: data.date_from,
                        end_date: data.date_to,
                        indicator: data.stp_indicator,
                        atoc_code: None,
                        performance_monitoring: None,
                        origin_location: None,
                        dest_location: None,
                    })
                }
                None => {
                    error!("Error parsing BS record (raw: {line})")
                }
            },
            "BX" => match from_bytes::<CIFBasicScheduleExtended>(line.as_bytes()).ok() {
                Some(data) => {
                    let schedule_temp = schedule.as_mut().expect("schedule should exist");

                    schedule_temp.atoc_code = Some(data.atoc_code);
                    schedule_temp.performance_monitoring = Some(data.performance_monitoring == "Y");
                }
                None => {
                    error!("Error parsing BX record (raw: {line})")
                }
            },
            "LO" => match from_bytes::<CIFOriginLocation>(line.as_bytes()).ok() {
                Some(mut data) => {
                    let location = data
                        .location
                        .trim()
                        .chars()
                        .take(7)
                        .collect::<String>()
                        .trim()
                        .to_string();

                    let schedule_temp = schedule.as_mut().expect("schedule should exist");

                    schedule_temp.origin_location = Some(
                        reference_locations
                            .get(&location)
                            .expect("tiploc should exist")
                            .clone(),
                    );

                    data.location = schedule_temp.origin_location.clone().unwrap();
                    locations.push(LocationTypes::Origin(data));
                }
                None => {
                    error!("Error parsing LO record (raw: {line})")
                }
            },
            "LI" => match from_bytes::<CIFIntermediateLocation>(line.as_bytes()).ok() {
                Some(data) => {
                    locations.push(LocationTypes::Intermediate(data));
                }
                None => {
                    error!("Error parsing LI record (raw: {line})")
                }
            },
            "LT" => {
                match from_bytes::<CIFTerminatingLocation>(line.as_bytes()).ok() {
                    Some(mut data) => {
                        let location = data
                            .location
                            .trim()
                            .chars()
                            .take(7)
                            .collect::<String>()
                            .trim()
                            .to_string(); // Strip away the suffix from the location

                        let schedule_temp = schedule.as_mut().expect("schedule should exist");

                        schedule_temp.dest_location = Some(
                            reference_locations
                                .get(&location)
                                .expect(format!("tiploc {location} should exist").as_str())
                                .clone(),
                        );

                        data.location = schedule_temp.dest_location.clone().unwrap();
                        locations.push(LocationTypes::Terminating(data));
                    }
                    None => {
                        error!("Error parsing LT record (raw: {line})")
                    }
                }
                let schedule_temp = schedule.as_ref().expect("schedule should exist");
                let db_schedule = Schedule::insert(
                    pool,
                    Schedule {
                        id: None,
                        uid: schedule_temp.uid.clone(),
                        identity: schedule_temp.identity.clone().unwrap(),
                        headcode: schedule_temp.headcode.clone(),
                        indicator: schedule_temp.indicator.clone(),
                        atoc_code: schedule_temp.atoc_code.clone().unwrap(),
                        performance_monitoring: schedule_temp.performance_monitoring.unwrap(),
                        origin_location: schedule_temp.origin_location.clone().unwrap(),
                        dest_location: schedule_temp
                            .dest_location
                            .clone()
                            .expect("Dest Location should exist"),
                        start_date: schedule_temp.start_date.clone(),
                        end_date: schedule_temp
                            .end_date
                            .clone()
                            .expect("End Date should exist"),
                    },
                )
                .await?;

                for loc in locations.clone() {
                    match loc {
                        LocationTypes::Origin(data) => {
                            ScheduleLocation::insert(
                                pool,
                                ScheduleLocation {
                                    id: None,
                                    location: data.location,
                                    scheduled_departure_time: Some(data.scheduled_departure_time),
                                    scheduled_arrival_time: None,
                                    scheduled_pass_time: None,
                                    public_departure_time: Some(data.public_departure_time),
                                    public_arrival_time: None,
                                    platform: data.platform,
                                    line: data.line,
                                    engineering_allowance: data.engineering_allowance,
                                    pathing_allowance: data.pathing_allowance,
                                    performance_allowance: data.performance_allowance,
                                    activity: Some(data.activity),
                                    schedule_id: db_schedule.id.unwrap(),
                                },
                            )
                            .await?;
                        }
                        LocationTypes::Intermediate(data) => {
                            ScheduleLocation::insert(
                                pool,
                                ScheduleLocation {
                                    id: None,
                                    location: data.location,
                                    scheduled_departure_time: data.scheduled_departure_time,
                                    scheduled_arrival_time: data.scheduled_arrival_time,
                                    scheduled_pass_time: data.scheduled_pass_time,
                                    public_departure_time: Some(data.public_departure_time),
                                    public_arrival_time: Some(data.public_arrival_time),
                                    platform: data.platform,
                                    line: data.line,
                                    engineering_allowance: data.engineering_allowance,
                                    pathing_allowance: data.pathing_allowance,
                                    performance_allowance: data.performance_allowance,
                                    activity: data.activity,
                                    schedule_id: db_schedule.id.unwrap(),
                                },
                            )
                            .await?;
                        }
                        LocationTypes::Terminating(data) => {
                            ScheduleLocation::insert(
                                pool,
                                ScheduleLocation {
                                    id: None,
                                    location: data.location,
                                    scheduled_departure_time: None,
                                    scheduled_arrival_time: Some(data.scheduled_arrival_time),
                                    scheduled_pass_time: None,
                                    public_departure_time: None,
                                    public_arrival_time: Some(data.public_arrival_time),
                                    platform: data.platform,
                                    line: None,
                                    engineering_allowance: None,
                                    pathing_allowance: None,
                                    performance_allowance: None,
                                    activity: Some(data.activity),
                                    schedule_id: db_schedule.id.unwrap(),
                                },
                            )
                            .await?;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    info!("Finished updating the schedule!");
    Ok(())
}

#[derive(Debug, Deserialize, FixedWidth)]
struct CIFHeader {
    #[fixed_width(range = "2..20")]
    mainframe_identity: String,
    #[fixed_width(range = "20..26")]
    extract_date: String,
    #[fixed_width(range = "26..30")]
    extract_time: String,
    #[fixed_width(range = "30..37")]
    current_file_reference: String,
    #[fixed_width(range = "37..44")]
    _last_file_reference: String,
    #[fixed_width(range = "44..45")]
    _update_indicator: String,
    #[fixed_width(range = "45..46")]
    version: String,
    #[fixed_width(range = "46..52")]
    _start_date: String,
    #[fixed_width(range = "52..58")]
    _end_date: String,
}

#[derive(Debug, Deserialize, FixedWidth)]
struct CIFBasicSchedule {
    #[fixed_width(range = "2..3")]
    transaction_type: String,
    #[fixed_width(range = "3..9")]
    train_uid: String,
    #[fixed_width(range = "9..15")]
    date_from: String,
    #[fixed_width(range = "15..21")]
    date_to: Option<String>,
    #[fixed_width(range = "21..28")]
    days_run: Option<String>,
    #[fixed_width(range = "28..29")]
    bank_holiday_running: Option<String>,
    #[fixed_width(range = "29..30")]
    train_status: Option<String>,
    #[fixed_width(range = "30..32")]
    train_category: Option<String>,
    #[fixed_width(range = "32..36")]
    train_identity: Option<String>,
    #[fixed_width(range = "36..40")]
    train_headcode: Option<String>,
    #[fixed_width(range = "40..41")]
    course_indicator: String,
    #[fixed_width(range = "41..49")]
    train_service_code: Option<String>,
    #[fixed_width(range = "49..50")]
    portion: Option<String>,
    #[fixed_width(range = "50..53")]
    power_type: Option<String>,
    #[fixed_width(range = "53..57")]
    timing_load: String,
    #[fixed_width(range = "57..60")]
    speed: Option<String>,
    #[fixed_width(range = "60..66")]
    operating_characteristic: Option<String>,
    #[fixed_width(range = "66..67")]
    seating_class: Option<String>,
    #[fixed_width(range = "67..68")]
    sleepers: Option<String>,
    #[fixed_width(range = "68..69")]
    reservations: Option<String>,
    #[fixed_width(range = "69..70")]
    connection_indicator: Option<String>,
    #[fixed_width(range = "70..73")]
    catering_code: Option<String>,
    #[fixed_width(range = "73..77")]
    service_branding: Option<String>,
    #[fixed_width(range = "79..80")]
    stp_indicator: String,
}

#[derive(Debug, Deserialize, FixedWidth)]
struct CIFBasicScheduleExtended {
    #[fixed_width(range = "2..6")]
    traction_class: Option<String>,
    #[fixed_width(range = "6..11")]
    uic_code: Option<i64>,
    #[fixed_width(range = "11..13")]
    atoc_code: String,
    #[fixed_width(range = "13..14")]
    performance_monitoring: String,
}

#[derive(Debug)]
struct StructuredSchedule {
    uid: String,
    identity: Option<String>,
    headcode: Option<String>,
    start_date: String,
    end_date: Option<String>,
    indicator: String,
    atoc_code: Option<String>,
    performance_monitoring: Option<bool>,
    origin_location: Option<String>,
    dest_location: Option<String>,
}

#[derive(Clone)]
enum LocationTypes {
    Origin(CIFOriginLocation),
    Intermediate(CIFIntermediateLocation),
    Terminating(CIFTerminatingLocation),
}

#[derive(Debug, Deserialize, FixedWidth, Clone)]
struct CIFOriginLocation {
    #[fixed_width(range = "2..10")]
    location: String,
    #[fixed_width(range = "10..15")]
    scheduled_departure_time: String,
    #[fixed_width(range = "15..19")]
    public_departure_time: String,
    #[fixed_width(range = "19..21")]
    platform: Option<String>,
    #[fixed_width(range = "21..24")]
    line: Option<String>,
    #[fixed_width(range = "24..26")]
    engineering_allowance: Option<String>,
    #[fixed_width(range = "26..28")]
    pathing_allowance: Option<String>,
    #[fixed_width(range = "28..40")]
    activity: String,
    #[fixed_width(range = "40..42")]
    performance_allowance: Option<String>,
}

#[derive(Debug, Deserialize, FixedWidth, Clone)]
struct CIFIntermediateLocation {
    #[fixed_width(range = "2..10")]
    location: String,
    #[fixed_width(range = "10..15")]
    scheduled_arrival_time: Option<String>,
    #[fixed_width(range = "15..20")]
    scheduled_departure_time: Option<String>,
    #[fixed_width(range = "20..25")]
    scheduled_pass_time: Option<String>,
    #[fixed_width(range = "25..29")]
    public_arrival_time: String,
    #[fixed_width(range = "29..33")]
    public_departure_time: String,
    #[fixed_width(range = "33..36")]
    platform: Option<String>,
    #[fixed_width(range = "36..39")]
    line: Option<String>,
    #[fixed_width(range = "39..42")]
    path: Option<String>,
    #[fixed_width(range = "42..54")]
    activity: Option<String>,
    #[fixed_width(range = "54..56")]
    engineering_allowance: Option<String>,
    #[fixed_width(range = "56..58")]
    pathing_allowance: Option<String>,
    #[fixed_width(range = "58..60")]
    performance_allowance: Option<String>,
}

#[derive(Debug, Deserialize, FixedWidth, Clone)]
struct CIFTerminatingLocation {
    #[fixed_width(range = "2..10")]
    location: String,
    #[fixed_width(range = "10..15")]
    scheduled_arrival_time: String,
    #[fixed_width(range = "15..19")]
    public_arrival_time: String,
    #[fixed_width(range = "19..21")]
    platform: Option<String>,
    #[fixed_width(range = "21..24")]
    path: Option<String>,
    #[fixed_width(range = "24..36")]
    activity: String,
}

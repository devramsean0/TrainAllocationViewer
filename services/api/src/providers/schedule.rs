use flate2::read::GzDecoder;
use log::{debug, error, info};
use rdkafka::message::ToBytes;
use sqlx::{Pool, Postgres};

use std::io::Read;

use crate::{
    sources::{nr_static::NRStatic, s3::S3Client},
    utils,
};
use futures::stream::TryStreamExt;
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

    info!("Finished updating the corpus!");
    Ok(())
}

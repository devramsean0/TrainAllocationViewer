use flate2::read::GzDecoder;
use log::{debug, info};
use rdkafka::message::ToBytes;
use sqlx::{Pool, Sqlite};

use std::io::Read;

use crate::sources::s3::S3Client;
use futures::stream::TryStreamExt;
use std::fs;

async fn decide_on_download() -> anyhow::Result<bool> {
    if !fs::exists("cache/CORPUSExtract.json.gz")? {
        info!("Downloading because cache not found");
        return Ok(true);
    } else {
        let client = S3Client::new()?;
        let s3_file_head = client.head("CORPUSExtract.json.gz").await?;
        let s3_md5 = s3_file_head.etag.unwrap().replace("\"", "");
        let md5 = crate::utils::md5::md5_of_file("cache/CORPUSExtract.json.gz").unwrap();

        debug!("S3: {}, local: {md5}", s3_md5);
        if s3_md5 == md5 {
            info!("Skipping download because cache matches");
            return Ok(false);
        }
        info!("Downloading because cache doesn't match");
        Ok(true)
    }
}
pub async fn update_corpus(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    if decide_on_download().await? {
        let client = S3Client::new()?;
        let s3_file_body = client.get("CORPUSExtract.json.gz").await?;

        let mut body_stream = s3_file_body.body;
        let mut bytes = Vec::new();
        while let Some(chunk) = body_stream.try_next().await? {
            bytes.extend_from_slice(&chunk);
        }
        fs::create_dir_all("cache")?;
        fs::write("cache/CORPUSExtract.json.gz", bytes.clone())?;
    }

    let file = fs::read("cache/CORPUSExtract.json.gz")?;
    let mut decompressor = GzDecoder::new(file.to_bytes());
    let mut json_string = String::new();
    decompressor.read_to_string(&mut json_string).unwrap();
    //fs::write("cache/CORPUSExtract.json", json_string.clone())?;

    let json = serde_json::from_str::<TiplocData>(json_string.as_str())?;
    info!("Corpus Length: {}", json.tiplocdata.len());

    crate::db::location::Location::insert_bulk(&pool, &json.tiplocdata).await?;
    Ok(())
}

// Format: {"NLC": 999078, "STANOX": " ", "TIPLOC": " ", "3ALPHA": " ", "UIC": " ", "NLCDESC": "AXIS/ACRES", "NLCDESC16": " "}

#[derive(serde::Deserialize)]
struct TiplocData {
    #[serde(rename = "TIPLOCDATA")]
    tiplocdata: Vec<LocationEntry>,
}

#[derive(serde::Deserialize)]
pub struct LocationEntry {
    #[serde(rename = "NLC")]
    pub nlc: i64,
    #[serde(rename = "STANOX")]
    pub stanox: Option<String>,
    #[serde(rename = "TIPLOC")]
    pub tiploc: Option<String>,
    #[serde(rename = "3ALPHA")]
    pub crs: Option<String>,
    #[serde(rename = "UIC")]
    pub uic: Option<String>,
    #[serde(rename = "NLCDESC")]
    pub nlcdesc: Option<String>,
    #[serde(rename = "AXIS/ACRES")]
    pub axis: Option<String>,
    #[serde(rename = "NLCDESC16")]
    pub nlcdesc16: Option<String>,
}

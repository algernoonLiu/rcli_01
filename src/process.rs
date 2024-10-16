use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Record {
    #[serde(rename = "Publish Time")]
    publish_time: String,
    title: String,
    position: u8,
    #[serde(rename = "Read Count")]
    read_count: u32,
    #[serde(rename = "Like Count")]
    like_count: u32,
    #[serde(rename = "View Count")]
    view_count: u32,
    link: String,
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Record = result?;
        ret.push(record);
    }

    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;

    Ok(())
}

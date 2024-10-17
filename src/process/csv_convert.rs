use csv::Reader;
// use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use crate::opts::OutputFormat;

///
/// Csv文件中的一条记录
// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct CsvRecord {
//     #[serde(rename = "Publish Time")]
//     publish_time: String,
//     title: String,
//     position: u8,
//     #[serde(rename = "Read Count")]
//     read_count: u32,
//     #[serde(rename = "Like Count")]
//     like_count: u32,
//     #[serde(rename = "View Count")]
//     view_count: u32,
//     link: String,
// }

///
/// 读取输入的csv文件，并转换为json写入到给定的文件
/// @param input 输入的csv文件路径
/// @param output 输出的json文件路径
/// @return Ok(()) if success, Err(anyhow::Error) if failed
///
// pub fn process_csv_0(input: &str, output: &str) -> anyhow::Result<()> {
//     let mut reader = Reader::from_path(input)?;
//     let mut ret = Vec::with_capacity(128);
//     for result in reader.deserialize() {
//         let record: CsvRecord = result?;
//         ret.push(record);
//     }

//     let json = serde_json::to_string_pretty(&ret)?;
//     fs::write(output, json)?;
//     Ok(())
// }

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        // headers.iter() 使用 hearders 的迭代器
        // record.iter() 使用 record 的迭代器
        // zip() 将两个迭代器合并成一个元组的迭代器 [(header, value), ...]
        // collect::<Value>() 将元组迭代器收集成一个 JSON Value
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    fs::write(output, content)?;

    Ok(())
}

// pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
//     let mut reader = Reader::from_path(input)?;
//     let mut ret = Vec::with_capacity(128);
//     let headers = reader.headers()?.clone();
//     for record in reader.records() {
//         let record = record?.clone();
//         let mut row_hash = HashMap::new();
//         for i in 0..headers.len() {
//             row_hash.insert(&headers[i], &record[i]);
//         }
//         ret.push(row_hash);
//     }

//     let json = serde_json::to_string_pretty(&ret)?;
//     fs::write(output, json)?;

//     Ok(())
// }

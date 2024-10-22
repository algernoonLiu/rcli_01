use clap::{Parser, ValueEnum};
use std::{fmt::Display, str::FromStr};

use crate::verify_file;

///
/// 读取输入的csv文件内容，将内容转换为json格式，并输出到指定文件
///
#[derive(Debug, Parser)]
pub struct CsvOpt {
    /// 输入文件路径
    #[arg(short, long, required = true, value_parser = verify_file)]
    pub input: String,
    /// 输出文件路径
    // #[arg(short, long, default_value = "output.json")]
    // pub output: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
    /// 分隔符
    #[arg(short, long, default_value = ",")]
    pub delimiter: char,
    /// 是否包含表头
    #[arg(long, default_value = "true")]
    pub header: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Json,
    Yaml,
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{}", Into::<&'static str>::into(*self))
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            why => Err(anyhow::anyhow!("Unsupported format: {}", why)),
        }
    }
}

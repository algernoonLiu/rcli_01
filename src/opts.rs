use clap::{Parser, ValueEnum};
use std::{fmt::Display, path::Path, str::FromStr};

///
/// 命令行参数操作结构体定义
/// cmd: Subcommand 命令行子命令
/// 
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

///
/// 子命令定义
/// csv: CsvOpt CSV操作
/// 
#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpt),
}

///
/// 读取输入的csv文件内容，将内容转换为json格式，并输出到指定文件
/// 
#[derive(Debug, Parser)]
pub struct CsvOpt {
    /// 输入文件路径
    #[arg(short, long, required = true, value_parser = verify_input_file)]
    pub input: String,
    /// 输出文件路径
    // #[arg(short, long, default_value = "output.json")]
    // pub output: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(value_enum, long, value_parser = parse_format, default_value = "json")]
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


///
/// 验证文件是否存在
/// 
/// return: Result<String, &'static str> 返回文件路径或错误信息
/// 
fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File dose not exist")
    }
}

mod base64;
mod csv;
mod genpass;

use std::path::Path;

use self::{csv::CsvOpt, genpass::GenpassOpt};
use clap::Parser;

pub use self::{base64::Base64Format, base64::Base64SubCommand, csv::OutputFormat};

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
/// genpass: GenpassOpt 生成密码
///
#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpt),
    #[command(name = "genpass", about = "Generate a random password")]
    Genpass(GenpassOpt),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

///
/// 验证文件是否存在
///
/// return: Result<String, &'static str> 返回文件路径或错误信息
///
fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File dose not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("not_exist_file"), Err("File dose not exist"));
    }
}
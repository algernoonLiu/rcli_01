mod csv;
mod genpass;

use self::{csv::CsvOpt, genpass::GenpassOpt};
use clap::Parser;

pub use self::csv::OutputFormat;

///
/// 命令行参数操作结构体定义
/// cmd: Subcommand 命令行子命令
///
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
}

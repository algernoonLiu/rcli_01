use std::{fmt, str::FromStr};
use clap::{Parser, ValueEnum};

use super::verify_input_file;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {

    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),

    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(Base64DecodeOpts),

}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    /// The string to encode
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    /// The base64 string to decode
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Base64Format {
    Standard,
    URLSafe,
}

fn parse_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl From<Base64Format> for &'static str {
    fn from(f: Base64Format) -> Self {
        match f {
            Base64Format::Standard => "standard",
            Base64Format::URLSafe => "urlsafe",
        }
    }
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::URLSafe),
            why => Err(anyhow::anyhow!("Unsupported format: {}", why)),
        }
    }
}

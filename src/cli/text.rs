use std::{fmt, path::PathBuf, str::FromStr};

use clap::{Parser, ValueEnum};

use crate::{verify_file, verify_path};


#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a privated/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signed message")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a new key")]
    Generate(TextKeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    /// The text file to sign
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    /// The key to use for signing
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    /// The output file to write the signature to
    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    /// The text file to verify
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    /// The signature file to verify against
    #[arg(short, long)]
    pub sig: String,
    /// The format of the signature file
    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
    /// The key to use for verification
    #[arg(short, long)]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
    /// The output file to write the key to
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TextSignFormat {
    Blake3,
    Ed22519,
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl fmt::Display for TextSignFormat{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed22519 => "ed22519",
        }
    }
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed22519" => Ok(TextSignFormat::Ed22519),
            why => Err(anyhow::anyhow!("Unsupported format: {}", why)),
        }
    }
}
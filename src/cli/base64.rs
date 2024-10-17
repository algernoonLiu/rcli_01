use clap::Parser;

#[derive(Debug, Parser)]
pub struct Base64Opt {
    /// Decode base64 string
    #[arg(long)]
    decode: bool,

    /// Encode string to base64
    #[arg(long)]
    encode: bool,

    /// Input file
    #[arg(long)]
    input: Option<String>,
}

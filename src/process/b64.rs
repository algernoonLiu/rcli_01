use crate::{cli::Base64Format, get_reader};
use base64::{engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD}, Engine as _};


pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::URLSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(&buf)?,
        Base64Format::URLSafe => URL_SAFE_NO_PAD.decode(&buf)?,
    };
    let decoded = String::from_utf8(decoded)?;
    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::URLSafe;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/base64.txt";
        let format = Base64Format::URLSafe;
        assert!(process_decode(input, format).is_ok());
    }

    #[test]
    fn test_decode() {
        let s = "W3BhY2thZ2VdDQpuYW1lID0gInJjbGkiDQp2ZXJzaW9uID0gIjAuMS4wIg0KZWRpdGlvbiA9ICIyMDIxIg0KYXV0aG9yID0gImx6bTkyMDUyMEBnbWFpbC5jb20iDQpsaWNlbnNlID0gIk1JVCINCg0KIyBTZWUgbW9yZSBrZXlzIGFuZCB0aGVpciBkZWZpbml0aW9ucyBhdCBodHRwczovL2RvYy5ydXN0LWxhbmcub3JnL2NhcmdvL3JlZmVyZW5jZS9tYW5pZmVzdC5odG1sDQoNCltkZXBlbmRlbmNpZXNdDQphbnlob3cgPSAiMS4wLjg5Ig0KYmFzZTY0ID0gIjAuMjIuMSINCmNsYXAgPSB7IHZlcnNpb24gPSAiNC41LjMiLCBmZWF0dXJlcyA9IFsiZGVyaXZlIl0gfQ0KY3N2ID0gIjEuMy4wIg0KcmFuZCA9ICIwLjguNSINCnNlcmRlID0geyB2ZXJzaW9uID0gIjEuMC4yMTAiLCBmZWF0dXJlcyA9IFsiZGVyaXZlIl0gfQ0Kc2VyZGVfanNvbiA9ICIxLjAuMTI4Ig0Kc2VyZGVfeWFtbCA9ICIwLjkuMzQiDQp6eGN2Ym4gPSAiMy4xLjAiDQo".to_string();
        let decoded = URL_SAFE_NO_PAD.decode(&s).unwrap();
        println!("{:?}", String::from_utf8(decoded));
    }
}
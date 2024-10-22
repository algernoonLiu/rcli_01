use std::{fs, io::Read, path::Path};

use crate::{get_reader, process::process_gen_pass, TextSignFormat};
use anyhow::{Ok, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

trait TextSign {
    // Sign the text from the reader and return the signature
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerify {
    // Verify the text from the reader with the given signature
    // fn verify<R: Read>(&self, reader: R, signature: &[u8]) -> Result<bool>;
    fn verify(&self, reader: impl Read, signature: &[u8]) -> Result<bool>;
}

trait KeyLoader {
    fn load(key: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

trait KeyGenerater {
    fn generate() -> Result<Vec<Vec<u8>>>;
}

struct Blake3 {
    key: [u8; 32],
}

struct Ed22519Signer {
    key: SigningKey,
}

struct Ed22519Verifier {
    key: VerifyingKey,
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        },
        TextSignFormat::Ed22519 => {
            let signer = Ed22519Signer::load(key)?;
            signer.sign(&mut reader)?
        },
    };
    let signed = URL_SAFE_NO_PAD.encode(signed);
    
    Ok(signed)
}

pub fn process_text_verify(input: &str, key: &str, sig: &str, format: TextSignFormat) -> Result<bool> {
    let mut reader = get_reader(input)?;
    let sig = URL_SAFE_NO_PAD.decode(sig)?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut reader, &sig)?
        },
        TextSignFormat::Ed22519 => {
            let verifier = Ed22519Verifier::load(key)?;
            verifier.verify(&mut reader, &sig)?
        },
    };
    Ok(verified)
}

pub fn process_text_keygen(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed22519 => Ed22519Signer::generate(),
    }
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        Ok(hash == sig)
    }
}

impl TextSign for Ed22519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(self.key.sign(&buf).to_bytes().to_vec())
    }
}

impl TextVerify for Ed22519Verifier {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        Ok(self.key.verify(&buf, &sig).is_ok())
    }
}

impl Blake3 {
    fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key: [u8; 32] = key.try_into()?;
        let signer = Blake3::new(key);
        Ok(signer)
    }
}

impl Ed22519Signer {
    fn new(key: SigningKey) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        let signer = Ed22519Signer::new(key);
        Ok(signer)
    }
}

impl Ed22519Verifier {
    fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        let verifier = Ed22519Verifier::new(key);
        Ok(verifier)
    }
}

impl KeyLoader for Blake3 {
    fn load(key: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(key)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed22519Signer {
    fn load(key: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(key)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed22519Verifier {
    fn load(key: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(key)?;
        Self::try_new(&key)
    }
}

impl KeyGenerater for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = process_gen_pass(32, true, 
            true, true, true)?;
        let vec = key.as_bytes().to_vec();
        Ok(vec![vec])
    }
}

impl KeyGenerater for Ed22519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        let pk = signing_key.verifying_key().as_bytes().to_vec();
        let sk = signing_key.to_bytes().to_vec();
        Ok(vec![sk, pk])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    ///
    /// 运行用例前先使用 generate 工具生成对应的key文件
    /// 
    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let blake3 = Blake3::load("fixtures/blake3.txt")?;
        let data = b"Hello, world!";
        let sig = blake3.sign(&mut &data[..])?;
        let verified = blake3.verify(&data[..], &sig)?;
        assert!(verified);
        Ok(())
    }

    /// 
    /// 运行用例前先使用 generate 工具生成对应的sk和pk文件
    /// 
    #[test]
    fn test_ed22519_sign_verify() -> Result<()> {
        let signer = Ed22519Signer::load("fixtures/ed25519.sk")?;
        let verifier = Ed22519Verifier::load("fixtures/ed25519.pk")?;
        let data = b"Hello, world!";
        let sig = signer.sign(&mut &data[..])?;
        let verified = verifier.verify(&data[..], &sig)?;
        assert!(verified);
        Ok(())
    }

}
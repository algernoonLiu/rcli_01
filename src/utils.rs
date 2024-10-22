
use std::{fs::File, io::Read, path::{Path, PathBuf}};

///
/// 验证文件是否存在
///
/// return: Result<String, &'static str> 返回文件路径或错误信息
///
pub fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File dose not exist")
    }
}

///
/// 验证文件是否存在
///
/// return: Result<PathBuf, &'static str> 返回文件路径或错误信息
///
pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let path = Path::new(path);
    if path.exists() && path.is_dir() {
        Ok(path.into())
    } else {
        Err("Path dose not exist")
    }
}

pub fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>>{
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

#[cfg(test)]
mod test {
    use crate::{get_reader, verify_file};

    #[test]
    fn test_verify_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not_exist_file"), Err("File dose not exist"));
    }

    #[test]
    fn test_get_reader() -> anyhow::Result<()> {
        let input = "fixtures/base64.txt";
        let mut reader = get_reader(input)?;
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        let buf = buf.trim();
        println!("{}", buf);
        Ok(())
    }
}
use anyhow::Ok;
use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBERS: &[u8] = b"123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*?";

pub fn process_gen_pass(
    length: u8,
    upper: bool,
    lower: bool,
    numbers: bool,
    symbols: bool,
) -> anyhow::Result<String> {
    let mut rng = rand::thread_rng();
    let mut pass = Vec::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER);
        pass.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
    }
    if lower {
        chars.extend_from_slice(LOWER);
        pass.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
    }
    if numbers {
        chars.extend_from_slice(NUMBERS);
        pass.push(*NUMBERS.choose(&mut rng).expect("NUMBER won't be empty"));
    }
    if symbols {
        chars.extend_from_slice(SYMBOLS);
        pass.push(*SYMBOLS.choose(&mut rng).expect("SYMBOL won't be empty"));
    }

    for _ in 0..(length - pass.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        pass.push(*c);
    }

    pass.shuffle(&mut rng);

    let password = String::from_utf8(pass)?;

    Ok(password)
}

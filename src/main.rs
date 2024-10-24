use std::fs;
use clap::Parser;
use rcli::{
    process_csv, 
    process_decode, process_encode, 
    process_gen_pass, 
    process_http_serve,
    process_text_keygen, process_text_sign, process_text_verify, 
    Base64SubCommand, HttpSubCommand, Opts, Subcommand, TextSubCommand
};
use zxcvbn::zxcvbn;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    match opts.cmd {
        // rcli csv -i input.csv -o output.json --header -d ','
        // 子命令csv处理
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        Subcommand::Genpass(opts) => {
            let password = process_gen_pass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.numbers,
                opts.symbols,
            )?;
            println!("{}", password);

            let estimate = zxcvbn(&password, &[]);
            eprintln!("password estimate score: {}", estimate.score());
        }
        Subcommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;
                println!("{}", decoded);
            }
        }
        Subcommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let sig = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", sig);
            }
            TextSubCommand::Verify(opts) => {
                let verified = process_text_verify(&opts.input, &opts.key, &opts.sig, opts.format)?;
                println!("{}", verified);
            }
            TextSubCommand::Generate(opts) =>   {
                let keys = process_text_keygen(opts.format)?;
                match opts.format {
                    rcli::TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &keys[0])?;
                    }
                    rcli::TextSignFormat::Ed22519 => {
                        let name = &opts.output;
                        fs::write(name.join("ed25519.sk"), &keys[0])?;
                        fs::write(name.join("ed25519.pk"), &keys[0])?;
                    }
                }
            }
            TextSubCommand::Encrypt(opts) => {
                println!("Implement text encrypt: {:?}", &opts)
            }
            TextSubCommand::Decrypt(opts) => {
                println!("Implement text decrypt: {:?}", &opts)
            }
        }
        Subcommand::Http(opts) => match opts {
            HttpSubCommand::Serve(opts) => {
                process_http_serve(opts.dir, opts.port).await?;
            }
        }
    }
    Ok(())
}

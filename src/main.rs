use clap::Parser;
use rcli::{process_csv, process_decode, process_encode, process_gen_pass, Base64SubCommand, Opts, Subcommand};

/// rcli csv -i input.csv -o output.json --header -d ','
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    // println!("{:?}", opts);
    match opts.cmd {
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
            process_gen_pass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.numbers,
                opts.symbols,
            )?;
        }
        Subcommand::Base64(subcmd) => {
            match subcmd {
                Base64SubCommand::Encode(opts) => {
                    process_encode(&opts.input, opts.format)?;
                }
                Base64SubCommand::Decode(opts) => {
                    process_decode(&opts.input, opts.format)?;
                }
            }
        }
    }
    Ok(())
}

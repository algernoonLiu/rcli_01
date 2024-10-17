use clap::Parser;
use rcli::{process_csv, Opts, Subcommand};

/// rcli csv -i input.csv -o output.json --header -d ','
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        // 子命令csv处理
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                // String::from("output.json")
                format!("output.{}", opts.format)
            };
            let _ = process_csv(&opts.input, output, opts.format);
        }
    }
    Ok(())
}

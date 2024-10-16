use clap::Parser;
use rcli::{process_csv, Opts, Subcommand};

/// rcli csv -i input.csv -o output.json --header -d ','
fn main() -> anyhow::Result<()> {
    let opt = Opts::parse();
    println!("{:?}", opt);
    match opt.cmd {
        Subcommand::Csv(csv_opt) => process_csv(&csv_opt.input, &csv_opt.output)?,
    }
    Ok(())
}

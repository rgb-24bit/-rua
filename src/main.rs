use crate::traits::SubCommandVariant;
use anyhow::Result;
use clap::{AppSettings, Clap};
mod time;
mod traits;
mod calc;

/// Simple command line tool written in rust.
#[derive(Clap)]
#[clap(version = "0.0.1", author = "rgb-24bit <rgb-24bit@foxmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Time(time::Time),
    Calc(calc::Calc),
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Time(t) => t.execute()?,
        SubCommand::Calc(c) => c.execute()?,
    }

    Ok(())
}

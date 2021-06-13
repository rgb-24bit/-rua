use clap::{AppSettings, Clap};

mod time;

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
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Time(t) => t.execute(),
    }
}

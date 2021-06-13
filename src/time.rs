use chrono::prelude::*;
use clap::Clap;
use std::time::{Duration, UNIX_EPOCH};

const TIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

/// Time conversion.
#[derive(Clap)]
pub struct Time {
    /// Convert unix timestamp to utc and local time.
    #[clap(short, long)]
    timestamps: Option<Vec<u64>>,
}

impl Time {
    pub fn execute(&self) {
        if let Some(ref ts) = self.timestamps {
            self.do_time(ts)
        }
    }

    fn do_time(&self, ts: &Vec<u64>) {
        for t in ts.clone() {
            let d = UNIX_EPOCH + Duration::from_secs(t);
            let u = DateTime::<Utc>::from(d);
            let l = DateTime::<Local>::from(d);

            println!("timestamp {}", t);
            println!("utc       {}", u.format(TIME_FORMAT).to_string());
            println!("local     {}", l.format(TIME_FORMAT).to_string());
        }
    }
}

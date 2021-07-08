use crate::traits::SubCommandVariant;
use anyhow::Result;
use chrono::prelude::*;
use clap::Clap;
use std::time::{Duration, UNIX_EPOCH};

const TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

/// Time conversion.
#[derive(Clap)]
pub struct Time {
    /// Convert unix timestamp to utc and local time.
    #[clap(short, long)]
    timestamps: Option<Vec<u64>>,
}

impl SubCommandVariant for Time {
    fn execute(&self) -> Result<()> {
        if let Some(ref ts) = self.timestamps {
            Time::do_time(ts);
        }
        Ok(())
    }
}

impl Time {
    fn do_time(ts: &[u64]) {
        for t in ts {
            let d = UNIX_EPOCH + Duration::from_secs(*t);
            let u = DateTime::<Utc>::from(d);
            let l = DateTime::<Local>::from(d);

            println!("timestamp {}", t);
            println!("utc       {}", u.format(TIME_FORMAT).to_string());
            println!("local     {}", l.format(TIME_FORMAT).to_string());
        }
    }
}

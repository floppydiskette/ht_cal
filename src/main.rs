use std::sync::Arc;
use lazy_static::lazy_static;
use tokio::sync::Mutex;

use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};
use crate::datetime::HDateTime;
use crate::history::HistoryData;

pub mod datetime;
pub mod history;
pub mod packet;
mod server;
mod backup;

lazy_static!{
    pub static ref HDATE: Arc<Mutex<HDateTime>> = Arc::new(Mutex::new(HDateTime::new()));
    pub static ref HISTORY: Arc<Mutex<HistoryData>> = Arc::new(Mutex::new(HistoryData::new()));
}

#[tokio::main]
async fn main() -> Result<()> {
    // if the files "hdt.bin" and "history.bin" exist, load them
    if std::path::Path::new("hdt.bin").exists() && std::path::Path::new("history.bin").exists() {
        let (hdt, history) = backup::load();
        *HDATE.lock().await = hdt;
        *HISTORY.lock().await = history;
    }

    println!("starting datetime thread...");
    tokio::spawn(async {
        datetime::HDateTime::ticking_thread(HDATE.clone()).await;
    });
    println!("done!");

    println!("starting query thread...");
    tokio::spawn(async {
        server::listen_query(HDATE.clone()).await;
    });
    println!("done!");

    println!("starting history thread...");
    tokio::spawn(async {
        server::listen_history(HISTORY.clone()).await;
    });
    println!("done!");

    println!("starting backup thread...");
    tokio::spawn(async {
        backup::backup_thread(HDATE.clone(), HISTORY.clone()).await;
    });
    println!("done!");

    println!("starting command interface...");
    let mut rl = Editor::<()>::new()?;
    loop {
        let readline = rl.readline("(a)dvance day, (s)tatus, (w)rite: ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match line.as_str() {
                    "a" => {
                        HDateTime::advance_day(HDATE.clone(), HISTORY.clone()).await;
                    }
                    "s" => {
                        let datetime = HDATE.lock().await;
                        println!("year: {}, month: {}, day: {}; {} seconds have elapsed today", datetime.year, datetime.format_month_name(), datetime.day, datetime.second);
                        println!("avg time per second: {}ms", datetime.time_since_second.num_milliseconds() as f64);
                        let history = HISTORY.lock().await;
                        println!("avg time per day: {}s", history.avg_seconds_per_day);
                    }
                    "w" => {
                        println!("to be implemented");
                    }
                    _ => {
                        println!("unknown command");
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C (will not exit to prevent data loss)");
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D (will not exit to prevent data loss)");
            }
            Err(err) => {
                println!("Error: {:?} (will not exit to prevent data loss)", err);
            }
        }
    }
}

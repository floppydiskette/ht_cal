use std::num::Wrapping;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use crate::datetime::{HDateTime, Month, MonthStatus};
use crate::history::HistoryData;

#[derive(Serialize, Deserialize)]
struct HDTSerializable {
    pub year: u128,
    pub month: (MonthStatus, Month),
    pub day: u8, // should theoretically only reach 24
    pub second: u128,
    pub time_since_second: u128,
}

pub fn save(hdt: HDateTime, history: HistoryData) {
    // serialise with rmp-serde
    let mut buf_hdt = Vec::new();
    let mut buf_history = Vec::new();
    let res = rmp_serde::encode::write(&mut buf_hdt, &HDTSerializable {
        year: hdt.year,
        month: hdt.month,
        day: hdt.day,
        second: hdt.second.0,
        time_since_second: hdt.time_since_second.num_milliseconds() as u128,
    });
    if res.is_err() {
        println!("save error: {:?}", res);
    }
    let res = rmp_serde::encode::write(&mut buf_history, &history);
    if res.is_err() {
        println!("save error: {:?}", res);
    }
    // write to file
    let res = std::fs::write("hdt.bin", buf_hdt);
    if res.is_err() {
        println!("save error: {:?}", res);
    }
    let res = std::fs::write("history.bin", buf_history);
    if res.is_err() {
        println!("save error: {:?}", res);
    }
    println!("save successful!");
}

pub fn load() -> (HDateTime, HistoryData) {
    // read from file
    let buf_hdt = std::fs::read("hdt.bin").unwrap();
    let buf_history = std::fs::read("history.bin").unwrap();
    // deserialise with rmp-serde
    let hdt: HDTSerializable = rmp_serde::decode::from_slice(&buf_hdt).unwrap();
    let history: HistoryData = rmp_serde::decode::from_slice(&buf_history).unwrap();
    // return
    let mut hdt_final = HDateTime::new();
    hdt_final.year = hdt.year;
    hdt_final.month = hdt.month;
    hdt_final.day = hdt.day;
    hdt_final.second = Wrapping(hdt.second);
    hdt_final.time_since_second = chrono::Duration::milliseconds(hdt.time_since_second as i64);
    (hdt_final, history)
}

pub async fn backup_thread(hdt: Arc<Mutex<HDateTime>>, history: Arc<Mutex<HistoryData>>) {
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        {
            let hdt = hdt.lock().await;
            let history = history.lock().await;
            save(*hdt, *history);
        }
    }
}
use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryData {
    pub last_ten_seconds_per_day: [u128; 10], // for the last ten days, how many seconds were within each one
    pub avg_seconds_per_day: u128, // average seconds per day (measured over the last ten days)
}

impl HistoryData {
    pub fn new() -> Self {
        Self {
            last_ten_seconds_per_day: [0; 10],
            avg_seconds_per_day: 0,
        }
    }

    pub fn add_day(&mut self, seconds: u128) {
        self.last_ten_seconds_per_day.rotate_right(1);
        self.last_ten_seconds_per_day[0] = seconds;
        // calculate average, removing any days where the value is 0
        let mut total = 0;
        let mut count = 0;
        for i in 0..10 {
            if self.last_ten_seconds_per_day[i] != 0 {
                total += self.last_ten_seconds_per_day[i];
                count += 1;
            }
        }
        self.avg_seconds_per_day = total / count;
    }
}
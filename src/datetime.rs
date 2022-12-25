use std::num::Wrapping;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Month {
    Zero,
    Niktvirin,
    Apress,
    Smosh,
    Funny,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonthStatus {
    Greater,
    Lesser,
}

#[derive(Clone, Copy)]
pub struct HDateTime {
    pub year: u128,
    pub month: (MonthStatus, Month),
    pub day: u8, // should theoretically only reach 24
    pub second: Wrapping<u128>,
    pub time_since_second: chrono::Duration,
    storage: u8,
}

impl HDateTime {
    pub fn new() -> HDateTime {
        HDateTime {
            year: 0,
            month: (MonthStatus::Greater, Month::Zero),
            day: 0,
            second: Wrapping(0),
            time_since_second: chrono::Duration::zero(),
            storage: 0,
        }
    }

    pub async fn ticking_thread(me: Arc<Mutex<Self>>) {
        loop {
            let time_at_start = chrono::Utc::now();
            // do a complex thing 1000 times and then log a second
            let mut storage = 0;
            for _ in 0..1500000 {
                let mut tmp = rand::random::<Wrapping<u8>>();
                tmp = tmp * tmp;
                tmp = tmp * tmp;
                tmp = tmp * tmp;
                tmp = tmp * tmp;
                tmp = tmp * tmp;
                let storage_low = storage;
                storage += storage_low;
            }
            {
                let mut me = me.lock().await;
                me.second += Wrapping(1);
                me.time_since_second = time_at_start - chrono::Utc::now();
                me.storage = storage; // just to kinda make sure it doesn't get optimised out
            }
        }
    }

    pub async fn advance_day(me: Arc<Mutex<Self>>, history: Arc<Mutex<crate::history::HistoryData>>) {
        let mut me = me.lock().await;
        history.lock().await.add_day(me.second.0);
        me.day += 1;
        me.second = Wrapping(0);
        // if the day is greater than 24, then advance the month
        if me.day > 24 {
            me.day = 0;
            // if lesser month, go to greater month; if greater month, go to next month (and switch to lesser month)
            match me.month.1 {
                Month::Zero => {
                    if me.month.0 == MonthStatus::Greater {
                        me.month.0 = MonthStatus::Lesser;
                    } else {
                        me.month.0 = MonthStatus::Greater;
                        me.month.1 = Month::Niktvirin;
                    }
                },
                Month::Niktvirin => {
                    if me.month.0 == MonthStatus::Greater {
                        me.month.0 = MonthStatus::Lesser;
                    } else {
                        me.month.0 = MonthStatus::Greater;
                        me.month.1 = Month::Apress;
                    }
                },
                Month::Apress => {
                    if me.month.0 == MonthStatus::Greater {
                        me.month.0 = MonthStatus::Lesser;
                    } else {
                        me.month.0 = MonthStatus::Greater;
                        me.month.1 = Month::Smosh;
                    }
                },
                Month::Smosh => {
                    if me.month.0 == MonthStatus::Greater {
                        me.month.0 = MonthStatus::Lesser;
                    } else {
                        me.month.0 = MonthStatus::Greater;
                        me.month.1 = Month::Funny;
                    }
                },
                Month::Funny => {
                    if me.month.0 == MonthStatus::Greater {
                        me.month.0 = MonthStatus::Lesser;
                    } else {
                        me.month.0 = MonthStatus::Greater;
                        me.month.1 = Month::Zero;
                        me.year += 1;
                    }
                },
            }
        }
    }

    pub async fn get_second(me: Arc<Mutex<Self>>) -> u128 {
        let me = me.lock().await;
        me.second.0
    }

    pub async fn get_week_number(me: Arc<Mutex<Self>>) -> u8 {
        let me = me.lock().await;
        me.day / 4
    }

    pub fn format_month_name(&self) -> String {
        let mut month_name = String::new();
        match self.month.0 {
            MonthStatus::Greater => month_name.push_str("Greater month of "),
            MonthStatus::Lesser => month_name.push_str("Lesser month of "),
        }
        match self.month.1 {
            Month::Zero => month_name.push_str("Zero"),
            Month::Niktvirin => month_name.push_str("Niktvirin"),
            Month::Apress => month_name.push_str("Apress"),
            Month::Smosh => month_name.push_str("Smosh"),
            Month::Funny => month_name.push_str("Funny"),
        }
        month_name
    }
}
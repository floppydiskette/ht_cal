use std::num::Wrapping;
use serde::{Serialize, Deserialize};
use crate::datetime::{Month, MonthStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketData {
    pub year: u128,
    pub month: (MonthStatus, Month),
    pub day: u8, // should theoretically only reach 24
    pub second: Wrapping<u128>,
    pub time_since_second_ms: u128,
    pub time_of_packet_sent_ms: u128,
}
pub mod datetime;
pub mod history;
pub mod packet;

use std::num::Wrapping;
use crate::datetime::{Month, MonthStatus};
use serde::{Serialize, Deserialize};
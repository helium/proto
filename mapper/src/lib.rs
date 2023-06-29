use chrono::{prelude::*, DateTime, NaiveDateTime};
pub use helium_proto::Message;
use serde::{Deserialize, Serialize};

mod attach;
pub use attach::*;

mod gps;
pub use gps::*;

mod scan;
pub use scan::*;

mod ports;
pub use ports::*;

pub mod scan_beacon;

pub type Result<T = ()> = std::result::Result<T, Error>;

use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("unexpected attach result str: {0}")]
    UnexpectedAttachResultStr(String),
    #[error("h3o: {0}")]
    H3o(#[from] h3o::error::InvalidLatLng),
    #[error("invalid attach result value: {value}")]
    InvalidAttachResultInt { value: i32 },
}

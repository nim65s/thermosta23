#![no_std]
#![feature(error_in_core)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
use actix_web::web::Bytes;

mod aht20;
mod relay;

use crate::aht20::{SensorErr, SensorResult};
use crate::relay::Relay;

use postcard::experimental::max_size::MaxSize;
use serde::{Deserialize, Serialize};

/// Errors handling in the code: fallible fonctions will return a Result over this
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("postcard error {0}")]
    Postcard(postcard::Error),

    #[error("sensor error {0:?}")]
    Sensor(SensorErr),

    #[cfg(any(feature = "std", feature = "wasm"))]
    #[error("Serde Json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

/// Errors handling on the wire: micro controllers can send those via Cmd.
#[derive(Deserialize, Serialize, MaxSize, Debug, PartialEq, Eq, Copy, Clone)]
pub enum CmdError {
    Esp(EspError),
}

#[derive(Deserialize, Serialize, MaxSize, Debug, PartialEq, Eq, Copy, Clone)]
pub enum EspError {
    CantEnableInterrupt,
}

#[repr(u8)]
#[derive(Deserialize, Serialize, MaxSize, Debug, Eq, PartialEq, Copy, Clone)]
pub enum Cmd {
    Get,
    Ping,
    Pong,
    Set(Relay),
    Status(Relay, SensorResult),
    Error(CmdError),
}

/// Heapless vector which can contain the biggest Cmd plus 2 additionnal bytes for COBS
pub type Vec = heapless::Vec<u8, { Cmd::POSTCARD_MAX_SIZE + 2 }>;

impl Cmd {
    pub fn to_vec(&self) -> Result<Vec> {
        postcard::to_vec_cobs(&self).map_err(Error::Postcard)
    }

    pub fn from_vec(value: &mut [u8]) -> Result<Self> {
        postcard::from_bytes_cobs(value).map_err(Error::Postcard)
    }

    #[cfg(feature = "std")]
    pub fn as_sse(self, event: &str) -> Result<Bytes> {
        let json = serde_json::to_string(&self)?;
        Ok(Bytes::from(std::format!(
            "event: {event}\ndata: {json}\n\n"
        )))
    }

    #[cfg(feature = "wasm")]
    pub fn from_sse(value: &web_sys::MessageEvent) -> Result<Option<Cmd>> {
        Ok(value
            .data()
            .as_string()
            .map(|data| serde_json::from_str(&data))
            .transpose()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aht20::SensorOk;
    extern crate std;

    #[test]
    fn cmd_to_vec_to_cmd() {
        let cmd_in = Cmd::Status(
            Relay::Open,
            SensorResult::Ok(SensorOk {
                h: u32::MAX,
                t: u32::MAX,
            }),
        );

        std::dbg!(cmd_in);
        let data = cmd_in.to_vec();
        assert!(data.is_ok(), "data is not ok: {data:?}");
        let mut data = data.unwrap();
        std::dbg!(&data);
        std::println!("data len: {}", data.len());
        std::println!("max len: {}", Cmd::POSTCARD_MAX_SIZE + 2);
        let cmd_out = Cmd::from_vec(data.as_mut());
        assert!(cmd_out.is_ok(), "cmd_out is not ok: {cmd_out:?}");
        let cmd_out = cmd_out.unwrap();
        std::dbg!(cmd_out);
        assert_eq!(cmd_out, cmd_in);
    }
}

//! A Board Support Package (BSP) which provides a type-safe API for the
//! [WEMOS/LOLIN D1 mini].
//!
//! [WEMOS/LOLIN D1 mini]: https://docs.wemos.cc/en/latest/d1/d1_mini.html

#![no_std]

mod pins;

pub use esp8266_hal::{self as hal, target};
pub use pins::Pins;

use esp8266_hal::{
    ehal::timer::{CountDown, Periodic},
    gpio::*,
    prelude::*,
};

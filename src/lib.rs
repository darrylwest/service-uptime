#![doc = include_str!("../README.md")]

pub mod counter;
pub mod uptime;

/// the current app version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

#[cfg(test)]
mod tests;

pub mod common;

mod dhcp_packet;
pub use dhcp_packet::*;

pub mod errors;

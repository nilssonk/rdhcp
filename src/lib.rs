#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

#[cfg(test)]
mod tests;

mod dhcp_packet;
pub use dhcp_packet::*;

pub mod errors;

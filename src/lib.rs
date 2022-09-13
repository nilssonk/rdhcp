#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

#[cfg(test)]
mod tests;

mod dhcp_packet;
pub use dhcp_packet::*;

mod dhcp_option;
pub use dhcp_option::DhcpOption;

pub mod errors;

#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

mod dhcp_packet;
pub use dhcp_packet::DhcpPacket;

mod dhcp_option;
pub use dhcp_option::DhcpOption;

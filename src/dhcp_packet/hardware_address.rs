#[derive(Debug, PartialEq, Eq)]
pub enum HardwareAddress {
    Ethernet([u8; 6]),
}

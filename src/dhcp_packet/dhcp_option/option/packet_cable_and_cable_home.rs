#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketCableAndCableHome;

impl PacketCableAndCableHome {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}

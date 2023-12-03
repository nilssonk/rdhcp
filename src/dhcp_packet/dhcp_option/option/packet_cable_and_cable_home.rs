#[derive(Debug, Clone, PartialEq)]
pub struct PacketCableAndCableHome;
        
impl PacketCableAndCableHome {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}

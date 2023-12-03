#[derive(Debug, Clone, PartialEq)]
pub struct PxeLinuxMagic;
        
impl PxeLinuxMagic {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}

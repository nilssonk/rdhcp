#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PxeLinuxMagic;

impl PxeLinuxMagic {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}

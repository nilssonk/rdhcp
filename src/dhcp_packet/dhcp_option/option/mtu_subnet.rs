#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MtuSubnet;

impl MtuSubnet {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}

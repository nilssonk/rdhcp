#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NisDomain;

impl NisDomain {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}

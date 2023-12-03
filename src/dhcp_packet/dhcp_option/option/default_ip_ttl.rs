#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefaultIpTtl;

impl DefaultIpTtl {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}

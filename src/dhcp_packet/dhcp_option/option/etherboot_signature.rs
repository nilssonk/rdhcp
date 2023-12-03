#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtherbootSignature;

impl EtherbootSignature {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}

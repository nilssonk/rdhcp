#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrailerEncapsulation;

impl TrailerEncapsulation {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}

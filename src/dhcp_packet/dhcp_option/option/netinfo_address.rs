#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetinfoAddress;

impl NetinfoAddress {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}

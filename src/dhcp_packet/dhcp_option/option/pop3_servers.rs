#[derive(Debug, Clone, PartialEq)]
pub struct Pop3Servers;

impl Pop3Servers {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}

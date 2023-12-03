#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrcServers;

impl IrcServers {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}

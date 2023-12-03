#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteStatisticsServerIpAddress;

impl RemoteStatisticsServerIpAddress {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}

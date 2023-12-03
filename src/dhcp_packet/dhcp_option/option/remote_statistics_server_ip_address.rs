#[derive(Debug, Clone, PartialEq)]
pub struct RemoteStatisticsServerIpAddress;
        
impl RemoteStatisticsServerIpAddress {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}

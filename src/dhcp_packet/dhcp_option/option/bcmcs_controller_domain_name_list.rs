#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BcmcsControllerDomainNameList;

impl BcmcsControllerDomainNameList {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}

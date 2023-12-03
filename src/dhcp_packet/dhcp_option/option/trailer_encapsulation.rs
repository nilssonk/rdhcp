#[derive(Debug, Clone, PartialEq)]
pub struct TrailerEncapsulation;
        
impl TrailerEncapsulation {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}

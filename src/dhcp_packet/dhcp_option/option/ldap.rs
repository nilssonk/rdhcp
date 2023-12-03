#[derive(Debug, Clone, PartialEq)]
pub struct Ldap;
        
impl Ldap {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}

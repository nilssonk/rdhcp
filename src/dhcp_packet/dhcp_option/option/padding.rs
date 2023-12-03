#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Padding {
    pub data: Vec<u8>,
}

impl Padding {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        Some(crate::DhcpOption::Padding(Padding {
            data: data.to_owned(),
        }))
    }
}

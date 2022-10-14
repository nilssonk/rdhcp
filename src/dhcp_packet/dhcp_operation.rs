#[derive(Debug, PartialEq, Eq)]
pub enum DhcpOperation {
    Request,
    Reply,
}

impl From<DhcpOperation> for u8 {
    #[inline(always)]
    fn from(op: DhcpOperation) -> Self {
        match op {
            DhcpOperation::Request => 1,
            DhcpOperation::Reply => 2,
        }
    }
}

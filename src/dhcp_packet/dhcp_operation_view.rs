use crate::errors::OutOfRange;
use crate::DhcpOperation;

use core::borrow::{Borrow, BorrowMut};

#[derive(Debug, PartialEq, Eq)]
pub struct DhcpOperationView<T> {
    data: T,
}

impl<T> DhcpOperationView<T> {
    #[inline(always)]
    pub(crate) fn from(data: T) -> Self {
        Self { data }
    }
}

impl<T> DhcpOperationView<T>
where
    T: Borrow<[u8]>,
{
    #[inline]
    pub fn try_get(&self) -> Result<DhcpOperation, OutOfRange> {
        match self.data.borrow()[0] {
            1 => Ok(DhcpOperation::Request),
            2 => Ok(DhcpOperation::Reply),
            _ => Err(OutOfRange),
        }
    }
}

impl<T> DhcpOperationView<T>
where
    T: BorrowMut<[u8]>,
{
    #[inline]
    pub fn set(&mut self, op: DhcpOperation) {
        self.data.borrow_mut()[0] = op.into();
    }
}

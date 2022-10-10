use crate::common::SizedInteger;

pub trait BeInteger {
    fn read_be(source: &[u8]) -> Self;
    fn write_be(&self, destination: &mut [u8]);
}

macro_rules! make_impl {
    {$t:ty} => {
    impl BeInteger for $t {
        #[inline(always)]
        fn read_be(source: & [u8]) -> Self {
            const SIZE: usize = <$t as SizedInteger>::SIZE;
            Self::from_be_bytes(source[..SIZE].try_into().unwrap())
        }

        #[inline(always)]
        fn write_be(&self, destination: &mut [u8]) {
            let data = self.to_be_bytes();
            destination.copy_from_slice(&data);
        }
    }
    };
}

make_impl! {u8}
make_impl! {u16}
make_impl! {u32}
make_impl! {u64}

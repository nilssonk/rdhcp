pub(crate) trait SizedInteger {
    const SIZE: usize;
}

impl SizedInteger for u8 {
    const SIZE: usize = 1;
}

impl SizedInteger for u16 {
    const SIZE: usize = 2;
}

impl SizedInteger for u32 {
    const SIZE: usize = 4;
}

impl SizedInteger for u64 {
    const SIZE: usize = 8;
}

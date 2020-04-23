use num::logic::traits::CountOnes;

macro_rules! impl_count_ones {
    ($t:ident) => {
        impl CountOnes for $t {
            #[inline]
            fn count_ones(self) -> u64 {
                u64::from($t::count_ones(self))
            }
        }
    };
}

impl_count_ones!(u8);
impl_count_ones!(u16);
impl_count_ones!(u32);
impl_count_ones!(u64);
impl_count_ones!(u128);
impl_count_ones!(usize);
impl_count_ones!(i8);
impl_count_ones!(i16);
impl_count_ones!(i32);
impl_count_ones!(i64);
impl_count_ones!(i128);
impl_count_ones!(isize);

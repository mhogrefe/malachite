use num::logic::traits::CountZeros;

macro_rules! impl_count_zeros {
    ($t:ident) => {
        impl CountZeros for $t {
            #[inline]
            fn count_zeros(self) -> u64 {
                u64::from($t::count_zeros(self))
            }
        }
    };
}
impl_count_zeros!(u8);
impl_count_zeros!(u16);
impl_count_zeros!(u32);
impl_count_zeros!(u64);
impl_count_zeros!(u128);
impl_count_zeros!(usize);
impl_count_zeros!(i8);
impl_count_zeros!(i16);
impl_count_zeros!(i32);
impl_count_zeros!(i64);
impl_count_zeros!(i128);
impl_count_zeros!(isize);

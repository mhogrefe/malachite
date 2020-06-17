use num::logic::traits::LeadingZeros;

macro_rules! impl_leading_zeros {
    ($t:ident) => {
        impl LeadingZeros for $t {
            #[inline]
            fn leading_zeros(self) -> u64 {
                u64::from($t::leading_zeros(self))
            }
        }
    };
}
impl_leading_zeros!(u8);
impl_leading_zeros!(u16);
impl_leading_zeros!(u32);
impl_leading_zeros!(u64);
impl_leading_zeros!(u128);
impl_leading_zeros!(usize);
impl_leading_zeros!(i8);
impl_leading_zeros!(i16);
impl_leading_zeros!(i32);
impl_leading_zeros!(i64);
impl_leading_zeros!(i128);
impl_leading_zeros!(isize);

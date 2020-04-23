use num::logic::traits::TrailingZeros;

macro_rules! impl_trailing_zeros {
    ($t:ident) => {
        impl TrailingZeros for $t {
            #[inline]
            fn trailing_zeros(self) -> u64 {
                u64::from($t::trailing_zeros(self))
            }
        }
    };
}

impl_trailing_zeros!(u8);
impl_trailing_zeros!(u16);
impl_trailing_zeros!(u32);
impl_trailing_zeros!(u64);
impl_trailing_zeros!(u128);
impl_trailing_zeros!(usize);
impl_trailing_zeros!(i8);
impl_trailing_zeros!(i16);
impl_trailing_zeros!(i32);
impl_trailing_zeros!(i64);
impl_trailing_zeros!(i128);
impl_trailing_zeros!(isize);

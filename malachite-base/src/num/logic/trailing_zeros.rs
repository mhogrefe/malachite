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
apply_to_primitive_ints!(impl_trailing_zeros);

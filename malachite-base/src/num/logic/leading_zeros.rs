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
apply_to_primitive_ints!(impl_leading_zeros);

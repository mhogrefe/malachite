use num::logic::traits::LeadingZeros;

macro_rules! impl_leading_zeros {
    ($t:ident) => {
        impl LeadingZeros for $t {
            /// This is a wrapper over the `leading_zeros` functions in the standard library, for
            /// example [this one](u32::leading_zeros).
            #[inline]
            fn leading_zeros(self) -> u64 {
                u64::from($t::leading_zeros(self))
            }
        }
    };
}
apply_to_primitive_ints!(impl_leading_zeros);

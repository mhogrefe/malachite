use num::logic::traits::CountZeros;

macro_rules! impl_count_zeros {
    ($t:ident) => {
        impl CountZeros for $t {
            /// This is a wrapper over the `count_zeros` functions in the standard library, for
            /// example [this one](u32::count_zeros).
            #[inline]
            fn count_zeros(self) -> u64 {
                u64::from($t::count_zeros(self))
            }
        }
    };
}
apply_to_primitive_ints!(impl_count_zeros);

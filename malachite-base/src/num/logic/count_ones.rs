use num::logic::traits::CountOnes;

macro_rules! impl_count_ones {
    ($t:ident) => {
        impl CountOnes for $t {
            /// This is a wrapper over the `count_ones` functions in the standard library, for
            /// example [this one](u32::count_ones).
            #[inline]
            fn count_ones(self) -> u64 {
                u64::from($t::count_ones(self))
            }
        }
    };
}
apply_to_primitive_ints!(impl_count_ones);

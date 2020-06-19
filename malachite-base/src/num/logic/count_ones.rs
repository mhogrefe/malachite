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
apply_to_primitive_ints!(impl_count_ones);

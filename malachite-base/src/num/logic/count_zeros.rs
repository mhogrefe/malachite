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
apply_to_primitive_ints!(impl_count_zeros);

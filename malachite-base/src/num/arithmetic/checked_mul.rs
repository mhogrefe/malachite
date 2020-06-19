use num::arithmetic::traits::CheckedMul;

macro_rules! impl_checked_mul {
    ($t:ident) => {
        impl CheckedMul<$t> for $t {
            type Output = $t;

            #[inline]
            fn checked_mul(self, other: $t) -> Option<$t> {
                $t::checked_mul(self, other)
            }
        }
    };
}
apply_to_primitive_ints!(impl_checked_mul);

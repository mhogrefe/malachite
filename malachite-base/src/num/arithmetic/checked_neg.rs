use num::arithmetic::traits::CheckedNeg;

macro_rules! impl_checked_neg {
    ($t:ident) => {
        impl CheckedNeg for $t {
            type Output = $t;

            #[inline]
            fn checked_neg(self) -> Option<$t> {
                $t::checked_neg(self)
            }
        }
    };
}
apply_to_primitive_ints!(impl_checked_neg);

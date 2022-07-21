use crate::num::arithmetic::traits::CheckedNeg;

macro_rules! impl_checked_neg {
    ($t:ident) => {
        impl CheckedNeg for $t {
            type Output = $t;

            /// This is a wrapper over the `checked_neg` functions in the standard library, for
            /// example [this one](u32::checked_neg).
            #[inline]
            fn checked_neg(self) -> Option<$t> {
                $t::checked_neg(self)
            }
        }
    };
}
apply_to_primitive_ints!(impl_checked_neg);

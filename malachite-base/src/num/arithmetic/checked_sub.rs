use num::arithmetic::traits::CheckedSub;

macro_rules! impl_checked_sub {
    ($t:ident) => {
        impl CheckedSub<$t> for $t {
            type Output = $t;

            /// This is a wrapper over the `checked_sub` functions in the standard library, for
            /// example [this one](u32::checked_sub).
            #[inline]
            fn checked_sub(self, other: $t) -> Option<$t> {
                $t::checked_sub(self, other)
            }
        }
    };
}
apply_to_primitive_ints!(impl_checked_sub);

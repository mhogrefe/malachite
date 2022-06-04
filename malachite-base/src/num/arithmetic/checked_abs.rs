use num::arithmetic::traits::CheckedAbs;

macro_rules! impl_checked_abs {
    ($t:ident) => {
        impl CheckedAbs for $t {
            type Output = $t;

            /// This is a wrapper over the `checked_abs` functions in the standard library, for
            /// example [this one](i32::checked_abs).
            #[inline]
            fn checked_abs(self) -> Option<$t> {
                $t::checked_abs(self)
            }
        }
    };
}
apply_to_signeds!(impl_checked_abs);

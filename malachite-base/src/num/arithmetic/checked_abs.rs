use num::arithmetic::traits::CheckedAbs;

macro_rules! impl_checked_abs {
    ($t:ident) => {
        impl CheckedAbs for $t {
            type Output = $t;

            #[inline]
            fn checked_abs(self) -> Option<$t> {
                $t::checked_abs(self)
            }
        }
    };
}
apply_to_signeds!(impl_checked_abs);

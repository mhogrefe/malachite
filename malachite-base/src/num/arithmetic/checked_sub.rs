use num::arithmetic::traits::CheckedSub;

macro_rules! impl_checked_sub {
    ($t:ident) => {
        impl CheckedSub<$t> for $t {
            type Output = $t;

            #[inline]
            fn checked_sub(self, other: $t) -> Option<$t> {
                $t::checked_sub(self, other)
            }
        }
    };
}
apply_to_primitive_ints!(impl_checked_sub);

use num::arithmetic::traits::CheckedDiv;

macro_rules! impl_checked_div {
    ($t:ident) => {
        impl CheckedDiv<$t> for $t {
            type Output = $t;

            #[inline]
            fn checked_div(self, other: $t) -> Option<$t> {
                $t::checked_div(self, other)
            }
        }
    };
}
apply_to_primitive_ints!(impl_checked_div);

use crate::num::arithmetic::traits::CheckedAdd;

macro_rules! impl_checked_add {
    ($t:ident) => {
        impl CheckedAdd<$t> for $t {
            type Output = $t;

            /// This is a wrapper over the `checked_add` functions in the standard library, for
            /// example [this one](u32::checked_add).
            #[inline]
            fn checked_add(self, other: $t) -> Option<$t> {
                $t::checked_add(self, other)
            }
        }
    };
}
apply_to_primitive_ints!(impl_checked_add);

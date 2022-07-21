use crate::num::arithmetic::traits::CheckedNextPowerOf2;

macro_rules! impl_checked_next_power_of_2 {
    ($t:ident) => {
        impl CheckedNextPowerOf2 for $t {
            type Output = $t;

            /// This is a wrapper over the `checked_next_power_of_two` functions in the standard
            /// library, for example [this one](u32::checked_next_power_of_two).
            #[inline]
            fn checked_next_power_of_2(self) -> Option<$t> {
                $t::checked_next_power_of_two(self)
            }
        }
    };
}
apply_to_unsigneds!(impl_checked_next_power_of_2);

use num::arithmetic::traits::CheckedNextPowerOf2;

macro_rules! impl_checked_next_power_of_2 {
    ($t:ident) => {
        impl CheckedNextPowerOf2 for $t {
            type Output = $t;

            #[inline]
            fn checked_next_power_of_2(self) -> Option<$t> {
                $t::checked_next_power_of_two(self)
            }
        }
    };
}
apply_to_unsigneds!(impl_checked_next_power_of_2);

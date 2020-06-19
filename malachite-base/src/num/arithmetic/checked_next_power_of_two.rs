use num::arithmetic::traits::CheckedNextPowerOfTwo;

macro_rules! impl_checked_next_power_of_two {
    ($t:ident) => {
        impl CheckedNextPowerOfTwo for $t {
            type Output = $t;

            #[inline]
            fn checked_next_power_of_two(self) -> Option<$t> {
                $t::checked_next_power_of_two(self)
            }
        }
    };
}
apply_to_unsigneds!(impl_checked_next_power_of_two);

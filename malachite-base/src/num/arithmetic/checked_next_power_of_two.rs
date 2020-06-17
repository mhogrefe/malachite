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
impl_checked_next_power_of_two!(u8);
impl_checked_next_power_of_two!(u16);
impl_checked_next_power_of_two!(u32);
impl_checked_next_power_of_two!(u64);
impl_checked_next_power_of_two!(u128);
impl_checked_next_power_of_two!(usize);

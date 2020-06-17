use num::arithmetic::traits::IsPowerOfTwo;

macro_rules! impl_is_power_of_two {
    ($t:ident) => {
        impl IsPowerOfTwo for $t {
            #[inline]
            fn is_power_of_two(&self) -> bool {
                $t::is_power_of_two(*self)
            }
        }
    };
}
impl_is_power_of_two!(u8);
impl_is_power_of_two!(u16);
impl_is_power_of_two!(u32);
impl_is_power_of_two!(u64);
impl_is_power_of_two!(u128);
impl_is_power_of_two!(usize);

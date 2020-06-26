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
apply_to_unsigneds!(impl_is_power_of_two);

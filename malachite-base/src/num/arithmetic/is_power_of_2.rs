use num::arithmetic::traits::IsPowerOf2;

macro_rules! impl_is_power_of_2 {
    ($t:ident) => {
        impl IsPowerOf2 for $t {
            #[inline]
            fn is_power_of_2(&self) -> bool {
                $t::is_power_of_two(*self)
            }
        }
    };
}
apply_to_unsigneds!(impl_is_power_of_2);

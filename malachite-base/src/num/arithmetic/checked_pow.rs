use num::arithmetic::traits::CheckedPow;
use num::conversion::traits::ExactFrom;

macro_rules! impl_checked_pow {
    ($t:ident) => {
        impl CheckedPow<u64> for $t {
            type Output = $t;

            #[inline]
            fn checked_pow(self, exp: u64) -> Option<$t> {
                $t::checked_pow(self, u32::exact_from(exp))
            }
        }
    };
}
apply_to_primitive_ints!(impl_checked_pow);

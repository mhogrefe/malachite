use num::arithmetic::traits::{ModPowerOfTwoMulAssign, ModPowerOfTwoPow, ModPowerOfTwoPowAssign};
use num::basic::integers::PrimitiveInt;
use num::logic::traits::BitIterable;

fn _mod_power_of_two_pow<T: ModPowerOfTwoMulAssign<T> + PrimitiveInt>(
    x: T,
    exp: u64,
    pow: u64,
) -> T {
    assert!(pow <= T::WIDTH);
    if pow == 0 {
        return T::ZERO;
    }
    let mut out = T::ONE;
    for bit in exp.bits().rev() {
        out.mod_power_of_two_mul_assign(out, pow);
        if bit {
            out.mod_power_of_two_mul_assign(x, pow);
        }
    }
    out
}

macro_rules! impl_mod_power_of_two_pow {
    ($t:ident) => {
        impl ModPowerOfTwoPow<u64> for $t {
            type Output = $t;

            /// Computes `self.pow(exp)` mod 2<sup>`pow`</sup>. Assumes the input is already reduced
            /// mod 2<sup>`pow`</sup>.
            ///
            /// //TODO complexity
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoPow;
            ///
            /// assert_eq!(5u8.mod_power_of_two_pow(13, 3), 5);
            /// assert_eq!(7u32.mod_power_of_two_pow(1000, 6), 1);
            /// ```
            #[inline]
            fn mod_power_of_two_pow(self, exp: u64, pow: u64) -> $t {
                _mod_power_of_two_pow(self, exp, pow)
            }
        }

        impl ModPowerOfTwoPowAssign<u64> for $t {
            /// Replaces `self` with `self.pow(exp)` mod 2<sup>`pow`</sup>. Assumes the input is
            /// already reduced mod 2<sup>`pow`</sup>.
            ///
            /// //TODO complexity
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoPowAssign;
            ///
            /// let mut n = 5u8;
            /// n.mod_power_of_two_pow_assign(13, 3);
            /// assert_eq!(n, 5);
            ///
            /// let mut n = 7u32;
            /// n.mod_power_of_two_pow_assign(1000, 6);
            /// assert_eq!(n, 1);
            /// ```
            #[inline]
            fn mod_power_of_two_pow_assign(&mut self, exp: u64, pow: u64) {
                *self = self.mod_power_of_two_pow(exp, pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_two_pow);

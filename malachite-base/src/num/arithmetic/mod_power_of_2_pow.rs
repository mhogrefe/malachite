use num::arithmetic::traits::{ModPowerOf2MulAssign, ModPowerOf2Pow, ModPowerOf2PowAssign};
use num::basic::integers::PrimitiveInt;
use num::logic::traits::BitIterable;

fn _mod_power_of_2_pow<T: ModPowerOf2MulAssign<T> + PrimitiveInt>(x: T, exp: u64, pow: u64) -> T {
    assert!(pow <= T::WIDTH);
    if pow == 0 {
        return T::ZERO;
    }
    let mut out = T::ONE;
    for bit in exp.bits().rev() {
        out.mod_power_of_2_mul_assign(out, pow);
        if bit {
            out.mod_power_of_2_mul_assign(x, pow);
        }
    }
    out
}

macro_rules! impl_mod_power_of_2_pow {
    ($t:ident) => {
        impl ModPowerOf2Pow<u64> for $t {
            type Output = $t;

            /// Computes `self.pow(exp)` mod 2<sup>`pow`</sup>. Assumes the input is already reduced
            /// mod 2<sup>`pow`</sup>.
            ///
            /// //TODO complexity
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOf2Pow;
            ///
            /// assert_eq!(5u8.mod_power_of_2_pow(13, 3), 5);
            /// assert_eq!(7u32.mod_power_of_2_pow(1000, 6), 1);
            /// ```
            #[inline]
            fn mod_power_of_2_pow(self, exp: u64, pow: u64) -> $t {
                _mod_power_of_2_pow(self, exp, pow)
            }
        }

        impl ModPowerOf2PowAssign<u64> for $t {
            /// Replaces `self` with `self.pow(exp)` mod 2<sup>`pow`</sup>. Assumes the input is
            /// already reduced mod 2<sup>`pow`</sup>.
            ///
            /// //TODO complexity
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOf2PowAssign;
            ///
            /// let mut n = 5u8;
            /// n.mod_power_of_2_pow_assign(13, 3);
            /// assert_eq!(n, 5);
            ///
            /// let mut n = 7u32;
            /// n.mod_power_of_2_pow_assign(1000, 6);
            /// assert_eq!(n, 1);
            /// ```
            #[inline]
            fn mod_power_of_2_pow_assign(&mut self, exp: u64, pow: u64) {
                *self = self.mod_power_of_2_pow(exp, pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_pow);

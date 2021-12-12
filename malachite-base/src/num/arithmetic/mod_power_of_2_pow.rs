use num::arithmetic::traits::{ModPowerOf2MulAssign, ModPowerOf2Pow, ModPowerOf2PowAssign};
use num::basic::integers::PrimitiveInt;
use num::logic::traits::BitIterable;

fn mod_power_of_2_pow<T: ModPowerOf2MulAssign<T> + PrimitiveInt>(x: T, exp: u64, pow: u64) -> T {
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

            /// Computes `self.pow(exp)` mod $2^p$. Assumes the input is already reduced mod $2^p$.
            ///
            /// $f(x, n, p) = y$, where $x, y < 2^p$ and $x^n \equiv y \mod 2^p$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `exp.significant_bits()`.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2_pow` module.
            #[inline]
            fn mod_power_of_2_pow(self, exp: u64, pow: u64) -> $t {
                mod_power_of_2_pow(self, exp, pow)
            }
        }

        impl ModPowerOf2PowAssign<u64> for $t {
            /// Replaces `self` with `self.pow(exp)` mod $2^p$. Assumes the input is already
            /// reduced mod $2^p$.
            ///
            /// $x \gets y$, where $x, y < 2^p$ and $x^n \equiv y \mod 2^p$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `exp.significant_bits()`.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2_pow` module.
            #[inline]
            fn mod_power_of_2_pow_assign(&mut self, exp: u64, pow: u64) {
                *self = self.mod_power_of_2_pow(exp, pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_pow);

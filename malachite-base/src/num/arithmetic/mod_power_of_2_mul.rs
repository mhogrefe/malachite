use crate::num::arithmetic::traits::{ModPowerOf2Mul, ModPowerOf2MulAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;

fn mod_power_of_2_mul<T: PrimitiveUnsigned>(x: T, other: T, pow: u64) -> T {
    assert!(pow <= T::WIDTH);
    x.wrapping_mul(other).mod_power_of_2(pow)
}

#[inline]
fn mod_power_of_2_mul_assign<T: PrimitiveUnsigned>(x: &mut T, other: T, pow: u64) {
    assert!(pow <= T::WIDTH);
    x.wrapping_mul_assign(other);
    x.mod_power_of_2_assign(pow);
}

macro_rules! impl_mod_power_of_2_mul {
    ($t:ident) => {
        impl ModPowerOf2Mul<$t> for $t {
            type Output = $t;

            /// Multiplies two numbers modulo a third number $2^k$. Assumes the inputs are already
            /// reduced modulo $2^k$.
            ///
            /// $f(x, y, k) = z$, where $x, y, z < 2^k$ and $xy \equiv z \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH`.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_mul#mod_power_of_2_mul).
            #[inline]
            fn mod_power_of_2_mul(self, other: $t, pow: u64) -> $t {
                mod_power_of_2_mul(self, other, pow)
            }
        }

        impl ModPowerOf2MulAssign<$t> for $t {
            /// Multiplies two numbers modulo a third number $2^k$, in place. Assumes the inputs
            /// are already reduced modulo $2^k$.
            ///
            /// $x \gets z$, where $x, y, z < 2^k$ and $xy \equiv z \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH`.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_mul#mod_power_of_2_mul_assign).
            #[inline]
            fn mod_power_of_2_mul_assign(&mut self, other: $t, pow: u64) {
                mod_power_of_2_mul_assign(self, other, pow)
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_mul);

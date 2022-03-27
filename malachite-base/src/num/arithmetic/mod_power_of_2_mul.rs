use num::arithmetic::traits::{ModPowerOf2Mul, ModPowerOf2MulAssign};
use num::basic::unsigneds::PrimitiveUnsigned;

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

            /// Computes `self * other` mod $2^p$. Assumes the inputs are already reduced mod $2^p$.
            ///
            /// $f(x, y, p) = z$, where $x, y, z < 2^p$ and $xy \equiv z \mod 2^p$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2_mul` module.
            #[inline]
            fn mod_power_of_2_mul(self, other: $t, pow: u64) -> $t {
                mod_power_of_2_mul(self, other, pow)
            }
        }

        impl ModPowerOf2MulAssign<$t> for $t {
            /// Replaces `self` with `self * other` mod $2^p$. Assumes the inputs are already
            /// reduced mod $2^p$.
            ///
            /// $x \gets z$, where $x, y, z < 2^p$ and $xy \equiv z \mod 2^p$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2_mul` module.
            #[inline]
            fn mod_power_of_2_mul_assign(&mut self, other: $t, pow: u64) {
                mod_power_of_2_mul_assign(self, other, pow)
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_mul);

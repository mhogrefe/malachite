use num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Neg, ModPowerOf2NegAssign};
use num::basic::integers::PrimitiveInt;

fn _mod_power_of_2_neg<T: ModPowerOf2<Output = T> + PrimitiveInt>(x: T, pow: u64) -> T {
    assert!(pow <= T::WIDTH);
    x.wrapping_neg().mod_power_of_2(pow)
}

fn _mod_power_of_2_neg_assign<T: PrimitiveInt>(x: &mut T, pow: u64) {
    assert!(pow <= T::WIDTH);
    x.wrapping_neg_assign();
    x.mod_power_of_2_assign(pow);
}

macro_rules! impl_mod_power_of_2_neg {
    ($t:ident) => {
        impl ModPowerOf2Neg for $t {
            type Output = $t;

            /// Computes `-self` mod $2^p$. Assumes the input is already reduced mod $2^p$.
            ///
            /// $f(x, p) = y$, where $x, y < 2^p$ and $-x \equiv y \mod 2^p$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2_neg` module.
            #[inline]
            fn mod_power_of_2_neg(self, pow: u64) -> $t {
                _mod_power_of_2_neg(self, pow)
            }
        }

        impl ModPowerOf2NegAssign for $t {
            /// Replaces `self` with `-self` mod $2^p$. Assumes the input is already reduced mod
            /// $2^p$.
            ///
            /// $x \gets y$, where $x, y < 2^p$ and $-x \equiv y \mod 2^p$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2_neg` module.
            #[inline]
            fn mod_power_of_2_neg_assign(&mut self, pow: u64) {
                _mod_power_of_2_neg_assign(self, pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_neg);

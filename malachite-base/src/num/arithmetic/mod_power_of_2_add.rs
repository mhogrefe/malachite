use num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Add, ModPowerOf2AddAssign};
use num::basic::integers::PrimitiveInt;

fn _mod_power_of_2_add<T: ModPowerOf2<Output = T> + PrimitiveInt>(x: T, other: T, pow: u64) -> T {
    assert!(pow <= T::WIDTH);
    x.wrapping_add(other).mod_power_of_2(pow)
}

fn _mod_power_of_2_add_assign<T: PrimitiveInt>(x: &mut T, other: T, pow: u64) {
    assert!(pow <= T::WIDTH);
    x.wrapping_add_assign(other);
    x.mod_power_of_2_assign(pow);
}

macro_rules! impl_mod_power_of_2_add {
    ($t:ident) => {
        impl ModPowerOf2Add<$t> for $t {
            type Output = $t;

            /// Computes `self + other` mod $2^p$. Assumes the inputs are already reduced mod $2^p$.
            ///
            /// $f(x, y, p) = z$, where $x, y, z < 2^p$ and $x + y \equiv z \mod 2^p$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2_add` module.
            #[inline]
            fn mod_power_of_2_add(self, other: $t, pow: u64) -> $t {
                _mod_power_of_2_add(self, other, pow)
            }
        }

        impl ModPowerOf2AddAssign<$t> for $t {
            /// Replaces `self` with `self + other` mod $2^p$. Assumes the inputs are already
            /// reduced mod $2^p$.
            ///
            /// $x \gets z$, where $x, y, z < 2^p$ and $x + y \equiv z \mod 2^p$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2_add` module.
            #[inline]
            fn mod_power_of_2_add_assign(&mut self, other: $t, pow: u64) {
                _mod_power_of_2_add_assign(self, other, pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_add);

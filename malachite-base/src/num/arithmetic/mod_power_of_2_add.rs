use num::arithmetic::traits::{ModPowerOf2Add, ModPowerOf2AddAssign};
use num::basic::unsigneds::PrimitiveUnsigned;

fn mod_power_of_2_add<T: PrimitiveUnsigned>(x: T, other: T, pow: u64) -> T {
    assert!(pow <= T::WIDTH);
    x.wrapping_add(other).mod_power_of_2(pow)
}

fn mod_power_of_2_add_assign<T: PrimitiveUnsigned>(x: &mut T, other: T, pow: u64) {
    assert!(pow <= T::WIDTH);
    x.wrapping_add_assign(other);
    x.mod_power_of_2_assign(pow);
}

macro_rules! impl_mod_power_of_2_add {
    ($t:ident) => {
        impl ModPowerOf2Add<$t> for $t {
            type Output = $t;

            /// Adds two numbers modulo a third number $2^k$. Assumes the inputs are already
            /// reduced modulo $2^k$.
            ///
            /// $f(x, y, k) = z$, where $x, y, z < 2^k$ and $x + y \equiv z \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH`.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_add#mod_power_of_2_add).
            #[inline]
            fn mod_power_of_2_add(self, other: $t, pow: u64) -> $t {
                mod_power_of_2_add(self, other, pow)
            }
        }

        impl ModPowerOf2AddAssign<$t> for $t {
            /// Adds two numbers modulo a third number $2^k$, in place. Assumes the inputs are
            /// already reduced modulo $2^k$.
            ///
            /// $x \gets z$, where $x, y, z < 2^k$ and $x + y \equiv z \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `Self::WIDTH`.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_add#mod_power_of_2_add_assign).
            #[inline]
            fn mod_power_of_2_add_assign(&mut self, other: $t, pow: u64) {
                mod_power_of_2_add_assign(self, other, pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_add);

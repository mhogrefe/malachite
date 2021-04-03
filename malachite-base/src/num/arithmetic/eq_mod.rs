use num::arithmetic::traits::{EqMod, Mod};
use num::basic::traits::Zero;

fn _eq_mod<U: Eq, S: Copy + Eq + Mod<S, Output = U> + Zero>(x: S, other: S, m: S) -> bool {
    x == other || m != S::ZERO && x.mod_op(m) == other.mod_op(m)
}

macro_rules! impl_eq_mod {
    ($t:ident) => {
        impl EqMod<$t> for $t {
            /// Returns whether a value is equivalent to another value mod a third value `m`; that
            /// is, whether `self` - `other` is a multiple of `m`.
            ///
            /// Two numbers are equal to each other mod 0 iff they are equal.
            ///
            /// $f(x, y, m) = (x \equiv y \mod m)$.
            ///
            /// $f(x, y, m) = (\exists k \in \Z \ x - y = km)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::eq_mod` module.
            #[inline]
            fn eq_mod(self, other: $t, m: $t) -> bool {
                _eq_mod(self, other, m)
            }
        }
    };
}
apply_to_primitive_ints!(impl_eq_mod);

use num::arithmetic::traits::{EqMod, Mod};
use num::basic::traits::Zero;

fn _eq_mod<T: Copy + Eq + Zero>(x: T, other: T, m: T) -> bool
where
    T: Mod<T>,
    <T as Mod>::Output: Eq,
{
    x == other || m != T::ZERO && x.mod_op(m) == other.mod_op(m)
}

macro_rules! impl_eq_mod {
    ($t:ident) => {
        impl EqMod<$t> for $t {
            /// Returns whether a value is equivalent to another value mod a third value `m`; that
            /// is, whether `self` - `other` is a multiple of `m`. Two numbers are equal to each
            /// other mod 0 iff they are equal.
            ///
            /// Time: Worst case O(1)
            ///
            /// Additional memory: Worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::EqMod;
            ///
            /// assert_eq!(123u16.eq_mod(223, 100), true);
            /// assert_eq!((-123i32).eq_mod(277, 100), true);
            /// assert_eq!((-123i64).eq_mod(278, 100), false);
            /// ```
            #[inline]
            fn eq_mod(self, other: $t, m: $t) -> bool {
                _eq_mod(self, other, m)
            }
        }
    };
}
apply_to_primitive_ints!(impl_eq_mod);

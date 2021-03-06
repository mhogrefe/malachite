use num::arithmetic::traits::{Abs, AbsAssign, UnsignedAbs, WrappingAbs};
use num::conversion::traits::WrappingFrom;

fn _unsigned_abs<U: WrappingFrom<S>, S: WrappingAbs<Output = S>>(x: S) -> U {
    U::wrapping_from(x.wrapping_abs())
}

macro_rules! impl_abs {
    ($u:ident, $s:ident) => {
        impl Abs for $s {
            type Output = $s;

            #[inline]
            fn abs(self) -> $s {
                $s::abs(self)
            }
        }

        impl AbsAssign for $s {
            /// Replaces `self` with its absolute value.
            ///
            /// $x \gets |x|$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::abs` module.
            #[inline]
            fn abs_assign(&mut self) {
                *self = self.abs();
            }
        }

        impl UnsignedAbs for $s {
            type Output = $u;

            /// Computes the absolute value of `self` and converts it to the unsigned type of the
            /// same width.
            ///
            /// Unlike regular `abs`, this function lets you take the absolute value of the smallest
            /// representable value of a signed type.
            ///
            /// $f(x) = |x|$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::abs` module.
            #[inline]
            fn unsigned_abs(self) -> $u {
                _unsigned_abs::<$u, $s>(self)
            }
        }
    };
}
apply_to_unsigned_signed_pair!(impl_abs);

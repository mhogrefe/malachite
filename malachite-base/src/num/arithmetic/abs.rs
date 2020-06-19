use num::arithmetic::traits::{Abs, AbsAssign, UnsignedAbs, WrappingAbs};
use num::conversion::traits::WrappingFrom;

#[inline]
pub fn _unsigned_abs<U, S>(x: S) -> U
where
    S: WrappingAbs<Output = S>,
    U: WrappingFrom<S>,
{
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
            /// Replace `self` with its absolute value.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::AbsAssign;
            ///
            /// let mut x = 0i8;
            /// x.abs_assign();
            /// assert_eq!(x, 0i8);
            ///
            /// let mut x = 100i64;
            /// x.abs_assign();
            /// assert_eq!(x, 100i64);
            ///
            /// let mut x = -100i64;
            /// x.abs_assign();
            /// assert_eq!(x, 100i64);
            /// ```
            #[inline]
            fn abs_assign(&mut self) {
                *self = self.abs();
            }
        }

        impl UnsignedAbs for $s {
            type Output = $u;

            /// Computes the absolute value of `self` and converts it to the unsigned type of the
            /// same width. Unlike regular `abs`, this function lets you take the absolute value of
            /// the smallest representable value of a signed type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::UnsignedAbs;
            ///
            /// assert_eq!(0i8.unsigned_abs(), 0u8);
            /// assert_eq!(100i64.unsigned_abs(), 100u64);
            /// assert_eq!((-100i64).unsigned_abs(), 100u64);
            /// assert_eq!((-128i8).unsigned_abs(), 128u8);
            /// ```
            #[inline]
            fn unsigned_abs(self) -> $u {
                _unsigned_abs::<$u, $s>(self)
            }
        }
    };
}
apply_to_unsigned_signed_pair!(impl_abs);

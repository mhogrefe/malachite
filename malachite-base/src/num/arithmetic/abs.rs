use num::arithmetic::traits::{Abs, AbsAssign, UnsignedAbs};
use num::basic::signeds::PrimitiveSigned;
use num::conversion::traits::WrappingFrom;

macro_rules! impl_abs {
    ($t:ident) => {
        impl Abs for $t {
            type Output = $t;

            #[inline]
            fn abs(self) -> $t {
                $t::abs(self)
            }
        }

        impl AbsAssign for $t {
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

        impl UnsignedAbs for $t {
            type Output = <$t as PrimitiveSigned>::UnsignedOfEqualWidth;

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
            fn unsigned_abs(self) -> <$t as PrimitiveSigned>::UnsignedOfEqualWidth {
                <$t as PrimitiveSigned>::UnsignedOfEqualWidth::wrapping_from($t::wrapping_abs(self))
            }
        }
    };
}

impl_abs!(i8);
impl_abs!(i16);
impl_abs!(i32);
impl_abs!(i64);
impl_abs!(i128);
impl_abs!(isize);

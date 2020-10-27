use std::ops::{BitOr, Shl, Shr};

use num::basic::integers::PrimitiveInt;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::{HasHalf, JoinHalves, SplitInHalf, WrappingFrom};

#[inline]
fn _join_halves<T: BitOr<Output = T> + From<H> + Shl<u64, Output = T>, H: PrimitiveInt>(
    upper: H,
    lower: H,
) -> T {
    T::from(upper) << H::WIDTH | T::from(lower)
}

#[inline]
fn _upper_half<T: Copy + Shr<u64, Output = T>, H: PrimitiveInt + WrappingFrom<T>>(x: &T) -> H {
    H::wrapping_from(*x >> H::WIDTH)
}

macro_rules! impl_half_traits {
    ($t:ident, $ht: ident) => {
        /// Implements `HasHalf` for unsigned primitive integers.
        impl HasHalf for $t {
            /// The primitive integer type whose width is half of `Self`.
            type Half = $ht;
        }

        /// Implements `JoinHalves` for unsigned primitive integers.
        impl JoinHalves for $t {
            /// Joins two unsigned integers to form an unsigned integer with twice the width.
            ///
            /// Let $W$ be `$t::WIDTH`.
            ///
            /// $f(x, y) = 2^{W/2} x + y$.
            ///
            /// # Worst-case complexity
            ///
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::conversion::traits::JoinHalves;
            ///
            /// assert_eq!(u16::join_halves(1, 2), 258);
            /// assert_eq!(u32::join_halves(0xabcd, 0x1234), 0xabcd1234);
            /// ```
            #[inline]
            fn join_halves(upper: Self::Half, lower: Self::Half) -> Self {
                _join_halves(upper, lower)
            }
        }

        /// Implements `SplitInHalf` for unsigned primitive integers.
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::conversion::traits::SplitInHalf;
        ///
        /// assert_eq!(258u16.split_in_half(), (1, 2));
        /// assert_eq!(0xabcd1234u32.split_in_half(), (0xabcd, 0x1234));
        /// ```
        impl SplitInHalf for $t {
            /// Extracts the lower, or least significant half, of and unsigned integer.
            ///
            /// Let $W$ be `$t::WIDTH`.
            ///
            /// $f(n) = m$, where $m < 2^{W/2}$ and $n + 2^{W/2} k = m$ for some $k \in Z$.
            ///
            /// # Worst-case complexity
            ///
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::conversion::traits::SplitInHalf;
            ///
            /// assert_eq!(258u16.lower_half(), 2);
            /// assert_eq!(0xabcd1234u32.lower_half(), 0x1234);
            /// ```
            #[inline]
            fn lower_half(&self) -> Self::Half {
                $ht::wrapping_from(*self)
            }

            /// Extracts the upper, or most significant half, of an unsigned integer.
            ///
            /// $f(n) = \lfloor \frac{n}{2^{W/2}} \rfloor$.
            ///
            /// # Worst-case complexity
            ///
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::conversion::traits::SplitInHalf;
            ///
            /// assert_eq!(258u16.upper_half(), 1);
            /// assert_eq!(0xabcd1234u32.upper_half(), 0xabcd);
            /// ```
            #[inline]
            fn upper_half(&self) -> Self::Half {
                _upper_half(self)
            }
        }
    };
}
impl_half_traits!(u16, u8);
impl_half_traits!(u32, u16);
impl_half_traits!(u64, u32);
impl_half_traits!(u128, u64);

#[inline]
fn wide_lower_half<T: PrimitiveUnsigned>(x: T) -> T {
    x.mod_power_of_two(T::WIDTH >> 1)
}

#[inline]
pub(crate) fn wide_upper_half<T: PrimitiveUnsigned>(x: T) -> T {
    x >> (T::WIDTH >> 1)
}

#[inline]
pub(crate) fn wide_split_in_half<T: PrimitiveUnsigned>(x: T) -> (T, T) {
    (wide_upper_half(x), wide_lower_half(x))
}

#[inline]
pub(crate) fn wide_join_halves<T: PrimitiveUnsigned>(hi: T, lo: T) -> T {
    (hi << (T::WIDTH >> 1)) | lo
}

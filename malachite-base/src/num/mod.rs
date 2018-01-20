use rand::distributions::range::SampleRange;
use rand::Rand;
use std;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::*;
use traits::{Named, NegativeOne, One, Zero};

//TODO docs
pub trait Walkable: Copy + Eq + Ord {
    fn increment(&mut self);

    fn decrement(&mut self);
}

//TODO docs
pub trait WrappingNeg {
    fn wrapping_neg(&self) -> Self;
}

//TODO docs
pub trait LeadingZeros {
    fn leading_zeros(&self) -> u32;
}

//TODO docs
pub trait SignificantBits {
    fn significant_bits(&self) -> u64;
}

//TODO docs
pub trait PrimitiveInteger
    : BitAccess
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Copy
    + Display
    + Debug
    + Eq
    + Hash
    + LeadingZeros
    + Named
    + One
    + Ord
    + Shl<u32, Output = Self>
    + ShlAssign<u32>
    + Shr<u32, Output = Self>
    + ShrAssign<u32>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Rem<Output = Self>
    + DivAssign
    + Not<Output = Self>
    + Rand
    + SampleRange
    + Walkable
    + WrappingNeg
    + Zero {
    const WIDTH: u32;
    const MIN: Self;
    const MAX: Self;

    fn from_u32(u: u32) -> Self;

    fn from_u64(u: u64) -> Self;
}

//TODO docs
pub trait PrimitiveUnsigned: PrimitiveInteger + SignificantBits {}

//TODO docs
pub trait PrimitiveSigned: PrimitiveInteger + NegativeOne {
    fn from_i32(i: i32) -> Self;

    fn from_i64(i: i64) -> Self;
}

/// This trait defines functions that access or modify individual bits in a value, indexed by a
/// `u64`.
pub trait BitAccess {
    /// Determines whether the bit at `index` is true or false.
    fn get_bit(&self, index: u64) -> bool;

    /// Sets the bit at `index` to true.
    fn set_bit(&mut self, index: u64);

    /// Sets the bit at `index` to false.
    fn clear_bit(&mut self, index: u64);

    /// Sets the bit at `index` to whichever value `bit` is.
    ///
    /// Time: worst case O(max(f(n), g(n))), where f(n) is the worst-case time complexity of
    ///     `Self::set_bit` and g(n) is the worst-case time complexity of `Self::clear_bit`.
    ///
    /// Additional memory: worst case O(max(f(n), g(n))), where f(n) is the worst-case
    ///     additional-memory complexity of `Self::set_bit` and g(n) is the worst-case
    ///     additional-memory complexity of `Self::clear_bit`.
    ///
    /// # Panics
    /// See panics for `set_bit` and `assign_bit`.
    fn assign_bit(&mut self, index: u64, bit: bool) {
        if bit {
            self.set_bit(index);
        } else {
            self.clear_bit(index);
        }
    }

    /// Sets the bit at `index` to the opposite of its previous value.
    ///
    /// Time: worst case O(f(n) + max(g(n), h(n))), where f(n) is the worst-case time complexity of
    ///     `Self::get_bit`, g(n) is the worst-case time complexity of `Self::set_bit`, and h(n) is
    ///     the worst-case time complexity of `Self::clear_bit`.
    ///
    /// Additional memory: worst case O(f(n) + max(g(n), h(n))), where f(n) is the worst-case
    ///     additional-memory complexity of `Self::get_bit`, g(n) is the worst-case
    ///     additional-memory complexity of `Self::set_bit`, and h(n) is the worst-case
    ///     additional-memory complexity of `Self::clear_bit`.
    ///
    /// # Panics
    /// See panics for `get_bit`, `set_bit` and `assign_bit`.
    fn flip_bit(&mut self, index: u64) {
        if self.get_bit(index) {
            self.clear_bit(index);
        } else {
            self.set_bit(index);
        }
    }
}

//TODO docs
macro_rules! common_traits {
    ($t: ident, $width: expr, $u: ident, $from_u32: expr, $from_u64: expr) => {
        //TODO docs
        impl Walkable for $t {
            fn increment(&mut self) {
                *self = self.wrapping_add(1);
            }

            fn decrement(&mut self) {
                *self = self.wrapping_sub(1);
            }
        }

        //TODO docs
        impl PrimitiveInteger for $t {
            const WIDTH: u32 = $width;
            const MIN: Self = std::$t::MIN;
            const MAX: Self = std::$t::MAX;

            fn from_u32($u: u32) -> Self {
                $from_u32
            }

            fn from_u64($u: u64) -> Self {
                $from_u64
            }
        }

        //TODO docs
        impl Named for $t {
            const NAME: &'static str = stringify!($t);
        }

        //TODO docs
        impl LeadingZeros for $t {
            fn leading_zeros(&self) -> u32 {
                $t::leading_zeros(*self)
            }
        }

        //TODO docs
        impl WrappingNeg for $t {
            fn wrapping_neg(&self) -> Self {
                $t::wrapping_neg(*self)
            }
        }
    }
}

//TODO docs
macro_rules! unsigned_traits {
    ($t: ident, $width: expr, $u: ident, $from_u32: expr, $from_u64: expr) => {
        common_traits!($t, $width, $u, $from_u32, $from_u64);

        //TODO docs and tests
        impl SignificantBits for $t {
            fn significant_bits(&self) -> u64 {
                (Self::WIDTH - self.leading_zeros()).into()
            }
        }

        //TODO docs
        impl PrimitiveUnsigned for $t {}

        /// Provides functions for accessing and modifying the `index`th bit of a primitive unsigned
        /// integer, or the coefficient of 2^<pow>`index`</pow> in its binary expansion.
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::BitAccess;
        ///
        /// let mut x = 0;
        /// x.assign_bit(2, true);
        /// x.assign_bit(5, true);
        /// x.assign_bit(6, true);
        /// assert_eq!(x, 100);
        /// x.assign_bit(2, false);
        /// x.assign_bit(5, false);
        /// x.assign_bit(6, false);
        /// assert_eq!(x, 0);
        ///
        /// let mut x = 0u64;
        /// x.flip_bit(10);
        /// assert_eq!(x, 1024);
        /// x.flip_bit(10);
        /// assert_eq!(x, 0);
        /// ```
        impl BitAccess for $t {
            /// Determines whether the `index`th bit of a primitive unsigned, or the coefficient of
            /// 2<pow>`index`</pow> in its binary expansion, is 0 or 1. `false` means 0, `true`
            /// means 1.
            ///
            /// Getting bits beyond the type's width is allowed; those bits are false.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::BitAccess;
            ///
            /// assert_eq!(123u8.get_bit(2), false);
            /// assert_eq!(123u16.get_bit(3), true);
            /// assert_eq!(123u32.get_bit(100), false);
            /// assert_eq!(1_000_000_000_000u64.get_bit(12), true);
            /// assert_eq!(1_000_000_000_000u64.get_bit(100), false);
            /// ```
            fn get_bit(&self, index: u64) -> bool {
                index < Self::WIDTH.into() && *self & (Self::ONE << index) != Self::ZERO
            }

            /// Sets the `index`th bit of a primitive unsigned integer, or the coefficient of
            /// 2<pow>`index`</pow> in its binary expansion, to 1.
            ///
            /// Setting bits beyond the type's width is disallowed.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `index >= Self::WIDTH`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::BitAccess;
            ///
            /// let mut x = 0u8;
            /// x.set_bit(2);
            /// x.set_bit(5);
            /// x.set_bit(6);
            /// assert_eq!(x, 100);
            /// ```
            fn set_bit(&mut self, index: u64) {
                if index < Self::WIDTH.into() {
                    *self |= Self::ONE << index;
                } else {
                    panic!(
                        "Cannot set bit {} in non-negative value of width {}",
                        index,
                        Self::WIDTH
                    );
                }
            }

            /// Sets the `index`th bit of a primitive unsigned integer, or the coefficient of
            /// 2<pow>`index`</pow> in its binary expansion, to 0.
            ///
            /// Clearing bits beyond the type's width is allowed; since those bits are already
            /// false, clearing them does nothing.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::BitAccess;
            ///
            /// let mut x = 0x7fu8;
            /// x.clear_bit(0);
            /// x.clear_bit(1);
            /// x.clear_bit(3);
            /// x.clear_bit(4);
            /// assert_eq!(x, 100);
            /// ```
            fn clear_bit(&mut self, index: u64) {
                if index < Self::WIDTH.into() {
                    *self &= !(Self::ONE << index);
                }
            }
        }
    }
}

//TODO docs
macro_rules! signed_traits {
    (
        $t: ident,
        $ut: ident,
        $width: expr,
        $u: ident,
        $from_u32: expr,
        $from_u64: expr,
        $i: ident,
        $from_i32: expr,
        $from_i64: expr
    ) => {
        common_traits!($t, $width, $u, $from_u32, $from_u64);

        //TODO docs
        impl PrimitiveSigned for $t {
            fn from_i32($i: i32) -> Self {
                $from_i32
            }

            fn from_i64($i: i64) -> Self {
                $from_i64
            }
        }

        //TODO docs and tests
        impl SignificantBits for $t {
            fn significant_bits(&self) -> u64 {
                (self.wrapping_abs() as $ut).significant_bits()
            }
        }

        /// Provides functions for accessing and modifying the `index`th bit of a primitive signed
        /// integer, or the coefficient of 2^<pow>`index`</pow> in its binary expansion.
        ///
        /// Negative integers are represented in two's complement.
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::BitAccess;
        ///
        /// let mut x = 0i8;
        /// x.assign_bit(2, true);
        /// x.assign_bit(5, true);
        /// x.assign_bit(6, true);
        /// assert_eq!(x, 100);
        /// x.assign_bit(2, false);
        /// x.assign_bit(5, false);
        /// x.assign_bit(6, false);
        /// assert_eq!(x, 0);
        ///
        /// let mut x = -0x100i16;
        /// x.assign_bit(2, true);
        /// x.assign_bit(5, true);
        /// x.assign_bit(6, true);
        /// assert_eq!(x, -156);
        /// x.assign_bit(2, false);
        /// x.assign_bit(5, false);
        /// x.assign_bit(6, false);
        /// assert_eq!(x, -256);
        ///
        /// let mut x = 0i32;
        /// x.flip_bit(10);
        /// assert_eq!(x, 1024);
        /// x.flip_bit(10);
        /// assert_eq!(x, 0);
        ///
        /// let mut x = -1i64;
        /// x.flip_bit(10);
        /// assert_eq!(x, -1025);
        /// x.flip_bit(10);
        /// assert_eq!(x, -1);
        /// ```
        impl BitAccess for $t {
            /// Determines whether the `index`th bit of a primitive signed integer, or the
            /// coefficient of 2<pow>`index`</pow> in its binary expansion, is 0 or 1. `false` means
            /// 0, `true` means 1.
            ///
            /// Negative integers are represented in two's complement.
            ///
            /// Accessing bits beyond the type's width is allowed; those bits are false if the
            /// integer is non-negative and true if it is negative.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::BitAccess;
            ///
            /// assert_eq!(123i8.get_bit(2), false);
            /// assert_eq!(123i16.get_bit(3), true);
            /// assert_eq!(123i32.get_bit(100), false);
            /// assert_eq!((-123i8).get_bit(0), true);
            /// assert_eq!((-123i16).get_bit(1), false);
            /// assert_eq!((-123i32).get_bit(100), true);
            /// assert_eq!(1_000_000_000_000i64.get_bit(12), true);
            /// assert_eq!(1_000_000_000_000i64.get_bit(100), false);
            /// assert_eq!((-1_000_000_000_000i64).get_bit(12), true);
            /// assert_eq!((-1_000_000_000_000i64).get_bit(100), true);
            /// ```
            fn get_bit(&self, index: u64) -> bool {
                if index < Self::WIDTH.into() {
                    self & (1 << index) != 0
                } else {
                    *self < 0
                }
            }

            /// Sets the `index`th bit of a primitive signed integer, or the coefficient of
            /// 2<pow>`index`</pow> in its binary expansion, to 1.
            ///
            /// Negative integers are represented in two's complement.
            ///
            /// Setting bits beyond the type's width is disallowed if the integer is non-negative;
            /// if it is negative, it's allowed but does nothing since those bits are already true.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `index >= Self::WIDTH && self >= 0`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::BitAccess;
            ///
            /// let mut x = 0i8;
            /// x.set_bit(2);
            /// x.set_bit(5);
            /// x.set_bit(6);
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -0x100i16;
            /// x.set_bit(2);
            /// x.set_bit(5);
            /// x.set_bit(6);
            /// assert_eq!(x, -156);
            /// ```
            fn set_bit(&mut self, index: u64) {
                if index < Self::WIDTH.into() {
                    *self |= 1 << index;
                } else if *self >= 0 {
                    panic!(
                        "Cannot set bit {} in non-negative value of width {}",
                        index,
                        Self::WIDTH
                    );
                }
            }

            /// Sets the `index`th bit of a primitive signed integer, or the coefficient of
            /// 2<pow>`index`</pow> in its binary expansion, to 0.
            ///
            /// Negative integers are represented in two's complement.
            ///
            /// Clearing bits beyond the type's width is disallowed if the integer is negative; if
            /// it is non-negative, it's allowed but does nothing since those bits are already
            /// false.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `index >= Self::WIDTH && self < 0`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::BitAccess;
            ///
            /// let mut x = 0x7fi8;
            /// x.clear_bit(0);
            /// x.clear_bit(1);
            /// x.clear_bit(3);
            /// x.clear_bit(4);
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -156i16;
            /// x.clear_bit(2);
            /// x.clear_bit(5);
            /// x.clear_bit(6);
            /// assert_eq!(x, -256);
            /// ```
            fn clear_bit(&mut self, index: u64) {
                if index < Self::WIDTH.into() {
                    *self &= !(1 << index);
                } else if *self < 0 {
                    panic!(
                        "Cannot clear bit {} in negative value of width {}",
                        index,
                        Self::WIDTH
                    );
                }
            }
        }
    }
}

unsigned_traits!(u8, 8, u, u as u8, u as u8);
unsigned_traits!(u16, 16, u, u as u16, u as u16);
unsigned_traits!(u32, 32, u, u, u as u32);
unsigned_traits!(u64, 64, u, u.into(), u);
signed_traits!(i8, u8, 8, u, u as i8, u as i8, i, i as i8, i as i8);
signed_traits!(i16, u16, 16, u, u as i16, u as i16, i, i as i16, i as i16);
signed_traits!(i32, u32, 32, u, u as i32, u as i32, i, i, i as i32);
signed_traits!(i64, u64, 64, u, u.into(), u as i64, i, i.into(), i);

pub fn get_lower(val: u64) -> u32 {
    val as u32
}

pub fn get_upper(val: u64) -> u32 {
    (val >> 32) as u32
}

pub fn make_u64(upper: u32, lower: u32) -> u64 {
    u64::from(upper) << 32 | u64::from(lower)
}

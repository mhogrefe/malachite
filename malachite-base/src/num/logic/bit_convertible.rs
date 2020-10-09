use std::ops::{BitOr, BitOrAssign, ShlAssign, ShrAssign};

use comparison::traits::Max;
use named::Named;
use num::arithmetic::traits::{Parity, WrappingNeg};
use num::basic::integers::PrimitiveInt;
use num::basic::traits::{NegativeOne, One, Zero};
use num::conversion::traits::WrappingFrom;
use num::logic::traits::{BitConvertible, LeadingZeros};

fn _to_bits_asc_unsigned<T: Copy + Eq + Parity + ShrAssign<u64> + Zero>(x: &T) -> Vec<bool> {
    let mut bits = Vec::new();
    let mut x = *x;
    while x != T::ZERO {
        bits.push(x.odd());
        x >>= 1;
    }
    bits
}

fn _to_bits_desc_unsigned<T: PrimitiveInt>(x: &T) -> Vec<bool> {
    let mut bits = Vec::new();
    if *x == T::ZERO {
        return bits;
    }
    bits.push(true);
    if *x == T::ONE {
        return bits;
    }
    let mut mask = T::power_of_two(T::WIDTH - LeadingZeros::leading_zeros(*x) - 2);
    while mask != T::ZERO {
        bits.push(*x & mask != T::ZERO);
        mask >>= 1;
    }
    bits
}

fn _from_bits_asc_unsigned<
    T: BitOrAssign<T> + Copy + Eq + Named + One + ShlAssign<u64> + Zero,
    I: Iterator<Item = bool>,
>(
    bits: I,
) -> T {
    let mut n = T::ZERO;
    let mut mask = T::ONE;
    for bit in bits {
        if mask == T::ZERO {
            assert!(!bit, "Bits cannot fit in integer of type {}", T::NAME);
        } else {
            if bit {
                n |= mask;
            }
            mask <<= 1;
        }
    }
    n
}

#[inline]
fn _from_bits_desc_unsigned<T: PrimitiveInt, I: Iterator<Item = bool>>(bits: I) -> T {
    let mut n = T::ZERO;
    let high_mask = T::power_of_two(T::WIDTH - 1);
    for bit in bits {
        if n & high_mask != T::ZERO {
            panic!("Bits cannot fit in integer of type {}", T::NAME);
        }
        n <<= 1;
        if bit {
            n |= T::ONE;
        }
    }
    n
}

macro_rules! impl_bit_convertible_unsigned {
    ($t:ident) => {
        impl BitConvertible for $t {
            /// Returns a `Vec` containing the bits of `self` in ascending order: least- to most-
            /// significant. If `self` is 0, the `Vec` is empty; otherwise, it ends with `true`.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = `self.significant_bits()`
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            ///
            /// assert_eq!(0u8.to_bits_asc(), &[]);
            /// assert_eq!(2u16.to_bits_asc(), &[false, true]);
            /// assert_eq!(123u32.to_bits_asc(), &[true, true, false, true, true, true, true]);
            /// ```
            #[inline]
            fn to_bits_asc(&self) -> Vec<bool> {
                _to_bits_asc_unsigned(self)
            }

            /// Returns a `Vec` containing the bits of `self` in descending order: most- to least-
            /// significant. If `self` is 0, the `Vec` is empty; otherwise, it begins with `true`.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = `self.significant_bits()`
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            ///
            /// assert_eq!(0u8.to_bits_desc(), &[]);
            /// assert_eq!(2u16.to_bits_desc(), &[true, false]);
            /// assert_eq!(123u32.to_bits_desc(), &[true, true, true, true, false, true, true]);
            /// ```
            #[inline]
            fn to_bits_desc(&self) -> Vec<bool> {
                _to_bits_desc_unsigned(self)
            }

            /// TODO doc
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            /// use std::iter::empty;
            ///
            /// assert_eq!(u8::from_bits_asc(empty()), 0);
            /// assert_eq!(u16::from_bits_asc([false, true, false].iter().cloned()), 2);
            /// assert_eq!(
            ///     u32::from_bits_asc(
            ///         [true, true, false, true, true, true, true].iter().cloned()
            ///     ),
            ///     123
            /// );
            /// ```
            #[inline]
            fn from_bits_asc<I: Iterator<Item = bool>>(bits: I) -> $t {
                _from_bits_asc_unsigned(bits)
            }

            /// TODO doc
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            /// use std::iter::empty;
            ///
            /// assert_eq!(u8::from_bits_desc(empty()), 0);
            /// assert_eq!(u16::from_bits_desc([false, true, false].iter().cloned()), 2);
            /// assert_eq!(
            ///     u32::from_bits_desc(
            ///         [true, true, true, true, false, true, true].iter().cloned()
            ///     ),
            ///     123
            /// );
            /// ```
            #[inline]
            fn from_bits_desc<I: Iterator<Item = bool>>(bits: I) -> $t {
                _from_bits_desc_unsigned(bits)
            }
        }
    };
}
apply_to_unsigneds!(impl_bit_convertible_unsigned);

fn _to_bits_asc_signed<T: Copy + Eq + NegativeOne + Ord + Parity + ShrAssign<u64> + Zero>(
    x: &T,
) -> Vec<bool> {
    let mut bits = Vec::new();
    let mut x = *x;
    if x >= T::ZERO {
        while x != T::ZERO {
            bits.push(x.odd());
            x >>= 1;
        }
        if !bits.is_empty() {
            bits.push(false);
        }
    } else {
        while x != T::NEGATIVE_ONE {
            bits.push(x.odd());
            x >>= 1;
        }
        bits.push(true);
    }
    bits
}

fn _to_bits_desc_signed<T: NegativeOne + PrimitiveInt>(x: &T) -> Vec<bool> {
    let mut bits = Vec::new();
    if *x >= T::ZERO {
        if *x == T::ZERO {
            return bits;
        }
        bits.push(false);
        bits.push(true);
        if *x == T::ONE {
            return bits;
        }
        let mut mask = T::power_of_two(T::WIDTH - LeadingZeros::leading_zeros(*x) - 2);
        while mask != T::ZERO {
            bits.push(*x & mask != T::ZERO);
            mask >>= 1;
        }
    } else {
        bits.push(true);
        if *x == T::NEGATIVE_ONE {
            return bits;
        }
        bits.push(false);
        if *x == T::NEGATIVE_ONE << 1 {
            return bits;
        }
        let mut mask = T::power_of_two(T::WIDTH - LeadingZeros::leading_zeros(!*x) - 2);
        while mask != T::ZERO {
            bits.push(*x & mask != T::ZERO);
            mask >>= 1;
        }
    }
    bits
}

fn _from_bits_asc_signed<
    U: BitOr<U, Output = U>
        + BitOrAssign<U>
        + Copy
        + Eq
        + Max
        + One
        + ShlAssign<u64>
        + WrappingNeg<Output = U>
        + Zero,
    S: Named + WrappingFrom<U>,
    I: Iterator<Item = bool>,
>(
    bits: I,
) -> S {
    let mut n = U::ZERO;
    let mut mask = U::ONE;
    let mut last_bit = false;
    for bit in bits {
        if mask == U::ZERO {
            assert_eq!(
                bit,
                last_bit,
                "Bits cannot fit in integer of type {}",
                S::NAME
            );
        } else {
            if bit {
                n |= mask;
            }
            mask <<= 1;
            last_bit = bit;
        }
    }
    if last_bit {
        S::wrapping_from(n | mask.wrapping_neg())
    } else {
        S::wrapping_from(n)
    }
}

#[inline]
fn _from_bits_desc_signed<U: PrimitiveInt, S: Named + WrappingFrom<U>, I: Iterator<Item = bool>>(
    bits: I,
) -> S {
    let mut n = U::ZERO;
    let high_mask = U::power_of_two(U::WIDTH - 2);
    let mut first = true;
    let mut sign_bit = false;
    for bit in bits {
        if first {
            sign_bit = bit;
            first = false;
        } else {
            if n & high_mask != U::ZERO {
                panic!("Bits cannot fit in integer of type {}", S::NAME);
            }
            n <<= 1;
            if bit != sign_bit {
                n |= U::ONE;
            }
        }
    }
    if sign_bit {
        S::wrapping_from(!n)
    } else {
        S::wrapping_from(n)
    }
}

macro_rules! impl_bit_convertible_signed {
    ($u:ident, $s:ident) => {
        impl BitConvertible for $s {
            /// Returns a `Vec` containing the bits of `self` in ascending order: least- to most-
            /// significant. If `self` is 0, the `Vec` is empty; otherwise, the last bit is the sign
            /// bit: `false` if `self` is non-negative and `true` if `self` is negative.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = `self.significant_bits()`
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            ///
            /// assert_eq!(0i8.to_bits_asc(), &[]);
            /// assert_eq!(2i16.to_bits_asc(), &[false, true, false]);
            /// assert_eq!(
            ///     (-123i32).to_bits_asc(),
            ///     &[true, false, true, false, false, false, false, true]
            /// );
            /// ```
            #[inline]
            fn to_bits_asc(&self) -> Vec<bool> {
                _to_bits_asc_signed(self)
            }

            /// Returns a `Vec` containing the bits of `self` in ascending order: most- to least-
            /// significant. If `self` is 0, the `Vec` is empty; otherwise, the first bit is the
            /// sign bit: `false` if `self` is non-negative and `true` if `self` is negative.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = `self.significant_bits()`
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            ///
            /// assert_eq!(0i8.to_bits_desc(), &[]);
            /// assert_eq!(2i16.to_bits_desc(), &[false, true, false]);
            /// assert_eq!(
            ///     (-123i32).to_bits_desc(),
            ///     &[true, false, false, false, false, true, false, true]
            /// );
            /// ```
            #[inline]
            fn to_bits_desc(&self) -> Vec<bool> {
                _to_bits_desc_signed(self)
            }

            /// TODO doc
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            /// use std::iter::empty;
            ///
            /// assert_eq!(i8::from_bits_asc(empty()), 0);
            /// assert_eq!(i16::from_bits_asc([false, true, false].iter().cloned()), 2);
            /// assert_eq!(
            ///     i32::from_bits_asc(
            ///         [true, false, true, false, false, false, false, true].iter().cloned()
            ///     ),
            ///     -123
            /// );
            /// ```
            #[inline]
            fn from_bits_asc<I: Iterator<Item = bool>>(bits: I) -> $s {
                _from_bits_asc_signed::<$u, $s, _>(bits)
            }

            /// TODO doc
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            /// use std::iter::empty;
            ///
            /// assert_eq!(i8::from_bits_desc(empty()), 0);
            /// assert_eq!(i16::from_bits_desc([false, true, false].iter().cloned()), 2);
            /// assert_eq!(
            ///     i32::from_bits_desc(
            ///         [true, false, false, false, false, true, false, true].iter().cloned()
            ///     ),
            ///     -123
            /// );
            /// ```
            #[inline]
            fn from_bits_desc<I: Iterator<Item = bool>>(bits: I) -> $s {
                _from_bits_desc_signed::<$u, $s, _>(bits)
            }
        }
    };
}
apply_to_unsigned_signed_pair!(impl_bit_convertible_signed);

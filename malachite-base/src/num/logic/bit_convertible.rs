use std::ops::ShrAssign;

use num::arithmetic::traits::Parity;
use num::basic::integers::PrimitiveInteger;
use num::basic::traits::{NegativeOne, Zero};
use num::conversion::traits::{ExactFrom, WrappingFrom};
use num::logic::traits::{BitConvertible, LeadingZeros};

pub fn _to_bits_asc_unsigned<T: Copy + Eq + Parity + ShrAssign<u64> + Zero>(x: &T) -> Vec<bool> {
    let mut bits = Vec::new();
    let mut x = *x;
    while x != T::ZERO {
        bits.push(x.odd());
        x >>= 1;
    }
    bits
}

pub fn _to_bits_desc_unsigned<T: PrimitiveInteger>(x: &T) -> Vec<bool> {
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

pub fn _from_bits_asc_unsigned<T: PrimitiveInteger>(bits: &[bool]) -> T {
    let width = usize::exact_from(T::WIDTH);
    if bits.len() > width {
        assert!(bits[width..].iter().all(|&bit| !bit));
    }
    let mut n = T::ZERO;
    let mut mask = T::ONE;
    for &bit in bits {
        if bit {
            n |= mask;
        }
        mask <<= 1;
    }
    n
}

pub fn _from_bits_desc_unsigned<T: PrimitiveInteger>(mut bits: &[bool]) -> T {
    let width = usize::exact_from(T::WIDTH);
    if bits.len() > width {
        let (bits_lo, bits_hi) = bits.split_at(bits.len() - width);
        assert!(bits_lo.iter().all(|&bit| !bit));
        bits = bits_hi;
    }
    let mut n = T::ZERO;
    for &bit in bits {
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

            /// Converts a slice of bits into a value. The input bits are in ascending order: least-
            /// to most-significant. The function panics if the input represents a number that can't
            /// fit in $t.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// where n = `bits.len()`
            ///
            /// # Panics
            /// Panics if the bits represent a value that isn't representable by $t.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            ///
            /// assert_eq!(u8::from_bits_asc(&[]), 0);
            /// assert_eq!(u16::from_bits_asc(&[false, true, false]), 2);
            /// assert_eq!(u32::from_bits_asc(&[true, true, false, true, true, true, true]), 123);
            /// ```
            #[inline]
            fn from_bits_asc(bits: &[bool]) -> $t {
                _from_bits_asc_unsigned(bits)
            }

            /// Converts a slice of bits into a value. The input bits are in descending order: most-
            /// to least-significant. The function panics if the input represents a number that
            /// can't fit in $t.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// where n = `bits.len()`
            ///
            /// # Panics
            /// Panics if the bits represent a value that isn't representable by $t.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            ///
            /// assert_eq!(u8::from_bits_desc(&[]), 0);
            /// assert_eq!(u16::from_bits_desc(&[false, true, false]), 2);
            /// assert_eq!(u32::from_bits_desc(&[true, true, true, true, false, true, true]), 123);
            /// ```
            #[inline]
            fn from_bits_desc(bits: &[bool]) -> $t {
                _from_bits_desc_unsigned(bits)
            }
        }
    };
}
impl_bit_convertible_unsigned!(u8);
impl_bit_convertible_unsigned!(u16);
impl_bit_convertible_unsigned!(u32);
impl_bit_convertible_unsigned!(u64);
impl_bit_convertible_unsigned!(u128);
impl_bit_convertible_unsigned!(usize);

pub fn _to_bits_asc_signed<T: Copy + Eq + NegativeOne + Ord + Parity + ShrAssign<u64> + Zero>(
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

pub fn _to_bits_desc_signed<T: NegativeOne + PrimitiveInteger>(x: &T) -> Vec<bool> {
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

pub fn _from_bits_asc_signed<T: PrimitiveInteger, U: PrimitiveInteger>(bits: &[bool]) -> T
where
    T: ExactFrom<U> + WrappingFrom<U>,
{
    match bits {
        &[] => T::ZERO,
        &[.., false] => T::exact_from(_from_bits_asc_unsigned::<U>(bits)),
        bits => {
            let trailing_trues = bits.iter().rev().take_while(|&&bit| bit).count();
            let significant_bits = bits.len() - trailing_trues;
            assert!(significant_bits < usize::exact_from(T::WIDTH));
            let mut u = !U::low_mask(u64::exact_from(significant_bits));
            let mut mask = U::ONE;
            for &bit in &bits[..significant_bits] {
                if bit {
                    u |= mask;
                }
                mask <<= 1;
            }
            T::wrapping_from(u)
        }
    }
}

pub fn _from_bits_desc_signed<T: PrimitiveInteger, U: PrimitiveInteger>(bits: &[bool]) -> T
where
    T: ExactFrom<U> + WrappingFrom<U>,
{
    match bits {
        &[] => T::ZERO,
        &[false, ..] => T::exact_from(_from_bits_desc_unsigned::<U>(bits)),
        bits => {
            let leading_trues = bits.iter().take_while(|&&bit| bit).count();
            let significant_bits = u64::exact_from(bits.len() - leading_trues);
            assert!(significant_bits < T::WIDTH);
            let mut mask = U::power_of_two(significant_bits);
            let mut u = !(mask - U::ONE);
            for &bit in &bits[leading_trues..] {
                mask >>= 1;
                if bit {
                    u |= mask;
                }
            }
            T::wrapping_from(u)
        }
    }
}

macro_rules! impl_bit_convertible_signed {
    ($t:ident, $u:ident) => {
        impl BitConvertible for $t {
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

            /// Converts a slice of bits into a value. The input bits are in ascending order: least-
            /// to most-significant. The function panics if the input represents a number that can't
            /// fit in $t.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// where n = `bits.len()`
            ///
            /// # Panics
            /// Panics if the bits represent a value that isn't representable by $t.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            ///
            /// assert_eq!(i8::from_bits_asc(&[]), 0);
            /// assert_eq!(i16::from_bits_asc(&[false, true, false]), 2);
            /// assert_eq!(
            ///     i32::from_bits_asc(&[true, false, true, false, false, false, false, true]),
            ///     -123
            /// );
            /// ```
            #[inline]
            fn from_bits_asc(bits: &[bool]) -> $t {
                _from_bits_asc_signed::<$t, $u>(bits)
            }

            /// Converts a slice of bits into a value. The input bits are in ascending order: least-
            /// to most-significant. The function panics if the input represents a number that can't
            /// fit in $t.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// where n = `bits.len()`
            ///
            /// # Panics
            /// Panics if the bits represent a value that isn't representable by $t.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            ///
            /// assert_eq!(i8::from_bits_desc(&[]), 0);
            /// assert_eq!(i16::from_bits_desc(&[false, true, false]), 2);
            /// assert_eq!(
            ///     i32::from_bits_desc(&[true, false, false, false, false, true, false, true]),
            ///     -123
            /// );
            /// ```
            #[inline]
            fn from_bits_desc(bits: &[bool]) -> $t {
                _from_bits_desc_signed::<$t, $u>(bits)
            }
        }
    };
}

impl_bit_convertible_signed!(i8, u8);
impl_bit_convertible_signed!(i16, u16);
impl_bit_convertible_signed!(i32, u32);
impl_bit_convertible_signed!(i64, u64);
impl_bit_convertible_signed!(i128, u128);
impl_bit_convertible_signed!(isize, usize);

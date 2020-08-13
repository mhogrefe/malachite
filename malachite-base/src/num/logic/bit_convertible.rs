use std::ops::{BitOr, BitOrAssign, ShlAssign, ShrAssign};

use comparison::traits::Max;
use named::Named;
use num::arithmetic::traits::{Parity, WrappingNeg};
use num::basic::integers::PrimitiveInteger;
use num::basic::traits::{NegativeOne, One, Zero};
use num::conversion::traits::{ExactFrom, WrappingFrom};
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

fn _to_bits_desc_unsigned<T: PrimitiveInteger>(x: &T) -> Vec<bool> {
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

fn _from_bits_asc_unsigned<T: PrimitiveInteger>(bits: &[bool]) -> T {
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

fn _from_bits_desc_unsigned<T: PrimitiveInteger>(mut bits: &[bool]) -> T {
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

fn _from_bit_iterator_asc_unsigned<T: Copy + Eq + Named + One + Zero, I: Iterator<Item = bool>>(
    bits: I,
) -> T
where
    T: BitOrAssign<T> + ShlAssign<u64>,
{
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
fn _from_bit_iterator_desc_unsigned<T: PrimitiveInteger, I: Iterator<Item = bool>>(bits: I) -> T {
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

            /// TODO doc
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            /// use std::iter::empty;
            ///
            /// assert_eq!(u8::from_bit_iterator_asc(empty()), 0);
            /// assert_eq!(u16::from_bit_iterator_asc([false, true, false].iter().cloned()), 2);
            /// assert_eq!(
            ///     u32::from_bit_iterator_asc(
            ///         [true, true, false, true, true, true, true].iter().cloned()
            ///     ),
            ///     123
            /// );
            /// ```
            #[inline]
            fn from_bit_iterator_asc<I: Iterator<Item = bool>>(bits: I) -> $t {
                _from_bit_iterator_asc_unsigned(bits)
            }

            /// TODO doc
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            /// use std::iter::empty;
            ///
            /// assert_eq!(u8::from_bit_iterator_desc(empty()), 0);
            /// assert_eq!(u16::from_bit_iterator_desc([false, true, false].iter().cloned()), 2);
            /// assert_eq!(
            ///     u32::from_bit_iterator_desc(
            ///         [true, true, true, true, false, true, true].iter().cloned()
            ///     ),
            ///     123
            /// );
            /// ```
            #[inline]
            fn from_bit_iterator_desc<I: Iterator<Item = bool>>(bits: I) -> $t {
                _from_bit_iterator_desc_unsigned(bits)
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

fn _to_bits_desc_signed<T: NegativeOne + PrimitiveInteger>(x: &T) -> Vec<bool> {
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

fn _from_bits_asc_signed<U: PrimitiveInteger, S: PrimitiveInteger>(bits: &[bool]) -> S
where
    S: ExactFrom<U> + WrappingFrom<U>,
{
    match bits {
        &[] => S::ZERO,
        &[.., false] => S::exact_from(_from_bits_asc_unsigned::<U>(bits)),
        bits => {
            let trailing_trues = bits.iter().rev().take_while(|&&bit| bit).count();
            let significant_bits = bits.len() - trailing_trues;
            assert!(significant_bits < usize::exact_from(S::WIDTH));
            let mut u = !U::low_mask(u64::exact_from(significant_bits));
            let mut mask = U::ONE;
            for &bit in &bits[..significant_bits] {
                if bit {
                    u |= mask;
                }
                mask <<= 1;
            }
            S::wrapping_from(u)
        }
    }
}

fn _from_bits_desc_signed<U: PrimitiveInteger, S: PrimitiveInteger>(bits: &[bool]) -> S
where
    S: ExactFrom<U> + WrappingFrom<U>,
{
    match bits {
        &[] => S::ZERO,
        &[false, ..] => S::exact_from(_from_bits_desc_unsigned::<U>(bits)),
        bits => {
            let leading_trues = bits.iter().take_while(|&&bit| bit).count();
            let significant_bits = u64::exact_from(bits.len() - leading_trues);
            assert!(significant_bits < S::WIDTH);
            let mut mask = U::power_of_two(significant_bits);
            let mut u = !(mask - U::ONE);
            for &bit in &bits[leading_trues..] {
                mask >>= 1;
                if bit {
                    u |= mask;
                }
            }
            S::wrapping_from(u)
        }
    }
}

fn _from_bit_iterator_asc_signed<
    U: Copy + Eq + Max + One + Zero,
    S: Named,
    I: Iterator<Item = bool>,
>(
    bits: I,
) -> S
where
    U: BitOr<U, Output = U> + BitOrAssign<U> + ShlAssign<u64> + WrappingNeg<Output = U>,
    S: WrappingFrom<U>,
{
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
fn _from_bit_iterator_desc_signed<U: PrimitiveInteger, S: Named, I: Iterator<Item = bool>>(
    bits: I,
) -> S
where
    S: WrappingFrom<U>,
{
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

            /// Converts a slice of bits into a value. The input bits are in ascending order: least-
            /// to most-significant. The function panics if the input represents a number that can't
            /// fit in $s.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// where n = `bits.len()`
            ///
            /// # Panics
            /// Panics if the bits represent a value that isn't representable by $s.
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
            fn from_bits_asc(bits: &[bool]) -> $s {
                _from_bits_asc_signed::<$u, $s>(bits)
            }

            /// Converts a slice of bits into a value. The input bits are in ascending order: least-
            /// to most-significant. The function panics if the input represents a number that can't
            /// fit in $s.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// where n = `bits.len()`
            ///
            /// # Panics
            /// Panics if the bits represent a value that isn't representable by $s.
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
            fn from_bits_desc(bits: &[bool]) -> $s {
                _from_bits_desc_signed::<$u, $s>(bits)
            }

            /// TODO doc
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            /// use std::iter::empty;
            ///
            /// assert_eq!(i8::from_bit_iterator_asc(empty()), 0);
            /// assert_eq!(i16::from_bit_iterator_asc([false, true, false].iter().cloned()), 2);
            /// assert_eq!(
            ///     i32::from_bit_iterator_asc(
            ///         [true, false, true, false, false, false, false, true].iter().cloned()
            ///     ),
            ///     -123
            /// );
            /// ```
            #[inline]
            fn from_bit_iterator_asc<I: Iterator<Item = bool>>(bits: I) -> $s {
                _from_bit_iterator_asc_signed::<$u, $s, _>(bits)
            }

            /// TODO doc
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            /// use std::iter::empty;
            ///
            /// assert_eq!(i8::from_bit_iterator_desc(empty()), 0);
            /// assert_eq!(i16::from_bit_iterator_desc([false, true, false].iter().cloned()), 2);
            /// assert_eq!(
            ///     i32::from_bit_iterator_desc(
            ///         [true, false, false, false, false, true, false, true].iter().cloned()
            ///     ),
            ///     -123
            /// );
            /// ```
            #[inline]
            fn from_bit_iterator_desc<I: Iterator<Item = bool>>(bits: I) -> $s {
                _from_bit_iterator_desc_signed::<$u, $s, _>(bits)
            }
        }
    };
}
apply_to_unsigned_signed_pair!(impl_bit_convertible_signed);

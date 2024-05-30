// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;
use crate::num::logic::traits::{BitConvertible, LeadingZeros};
use alloc::vec::Vec;

fn to_bits_asc_unsigned<T: PrimitiveUnsigned>(x: &T) -> Vec<bool> {
    let mut bits = Vec::new();
    let mut x = *x;
    while x != T::ZERO {
        bits.push(x.odd());
        x >>= 1;
    }
    bits
}

fn to_bits_desc_unsigned<T: PrimitiveUnsigned>(x: &T) -> Vec<bool> {
    let mut bits = Vec::new();
    if *x == T::ZERO {
        return bits;
    }
    bits.push(true);
    if *x == T::ONE {
        return bits;
    }
    let mut mask = T::power_of_2(T::WIDTH - LeadingZeros::leading_zeros(*x) - 2);
    while mask != T::ZERO {
        bits.push(*x & mask != T::ZERO);
        mask >>= 1;
    }
    bits
}

fn from_bits_asc_unsigned<T: PrimitiveUnsigned, I: Iterator<Item = bool>>(bits: I) -> T {
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
fn from_bits_desc_unsigned<T: PrimitiveUnsigned, I: Iterator<Item = bool>>(bits: I) -> T {
    let mut n = T::ZERO;
    let high_mask = T::power_of_2(T::WIDTH - 1);
    for bit in bits {
        assert!(
            n & high_mask == T::ZERO,
            "Bits cannot fit in integer of type {}",
            T::NAME
        );
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
            /// Returns a [`Vec`] containing the bits of a number in ascending order: least- to
            /// most-significant.
            ///
            /// If the number is 0, the [`Vec`] is empty; otherwise, it ends with `true`.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::bit_convertible#to_bits_asc).
            #[inline]
            fn to_bits_asc(&self) -> Vec<bool> {
                to_bits_asc_unsigned(self)
            }

            /// Returns a [`Vec`] containing the bits of a number in descending order: most- to
            /// least-significant.
            ///
            /// If the number is 0, the [`Vec`] is empty; otherwise, it begins with `true`.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::bit_convertible#to_bits_desc).
            #[inline]
            fn to_bits_desc(&self) -> Vec<bool> {
                to_bits_desc_unsigned(self)
            }

            /// Converts an iterator of bits into a number. The bits should be in ascending order
            /// (least- to most-significant).
            ///
            /// The function panics if the input represents a number that can't fit in the output
            /// type.
            ///
            /// $$
            /// f((b_i)_ {i=0}^{k-1}) = \sum_{i=0}^{k-1}2^i \[b_i\],
            /// $$
            /// where braces denote the Iverson bracket, which converts a bit to 0 or 1.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `bits.count()`.
            ///
            /// # Panics
            /// Panics if the bits represent a value that isn't representable by the output type.
            ///
            /// # Examples
            /// See [here](super::bit_convertible#from_bits_asc).
            #[inline]
            fn from_bits_asc<I: Iterator<Item = bool>>(bits: I) -> $t {
                from_bits_asc_unsigned(bits)
            }

            /// Converts an iterator of bits into a number. The bits should be in descending order
            /// (most- to least-significant).
            ///
            /// The function panics if the input represents a number that can't fit in the output
            /// type.
            ///
            /// $$
            /// f((b_i)_ {i=0}^{k-1}) = \sum_{i=0}^{k-1}2^{k-i-1} \[b_i\],
            /// $$
            /// where braces denote the Iverson bracket, which converts a bit to 0 or 1.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `bits.count()`.
            ///
            /// # Panics
            /// Panics if the bits represent a value that isn't representable by the output type.
            ///
            /// # Examples
            /// See [here](super::bit_convertible#from_bits_desc).
            #[inline]
            fn from_bits_desc<I: Iterator<Item = bool>>(bits: I) -> $t {
                from_bits_desc_unsigned(bits)
            }
        }
    };
}
apply_to_unsigneds!(impl_bit_convertible_unsigned);

fn to_bits_asc_signed<T: PrimitiveSigned>(x: &T) -> Vec<bool> {
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

fn to_bits_desc_signed<T: PrimitiveSigned>(x: &T) -> Vec<bool> {
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
        let mut mask = T::power_of_2(T::WIDTH - LeadingZeros::leading_zeros(*x) - 2);
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
        let mut mask = T::power_of_2(T::WIDTH - LeadingZeros::leading_zeros(!*x) - 2);
        while mask != T::ZERO {
            bits.push(*x & mask != T::ZERO);
            mask >>= 1;
        }
    }
    bits
}

fn from_bits_asc_signed<
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + WrappingFrom<U>,
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
fn from_bits_desc_signed<
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + WrappingFrom<U>,
    I: Iterator<Item = bool>,
>(
    bits: I,
) -> S {
    let mut n = U::ZERO;
    let high_mask = U::power_of_2(U::WIDTH - 2);
    let mut first = true;
    let mut sign_bit = false;
    for bit in bits {
        if first {
            sign_bit = bit;
            first = false;
        } else {
            assert!(
                n & high_mask == U::ZERO,
                "Bits cannot fit in integer of type {}",
                S::NAME
            );
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
            /// Returns a [`Vec`] containing the bits of a number in ascending order: least- to
            /// most-significant.
            ///
            /// If the number is 0, the [`Vec`] is empty; otherwise, the last bit is the sign bit:
            /// `false` if the number is non-negative and `true` if it is negative.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::bit_convertible#to_bits_asc).
            #[inline]
            fn to_bits_asc(&self) -> Vec<bool> {
                to_bits_asc_signed(self)
            }

            /// Returns a [`Vec`] containing the bits of a number in ascending order: most- to
            /// least-significant.
            ///
            /// If the number is 0, the [`Vec`] is empty; otherwise, the first bit is the sign bit:
            /// `false` if the number is non-negative and `true` if it is negative.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::bit_convertible#to_bits_desc).
            #[inline]
            fn to_bits_desc(&self) -> Vec<bool> {
                to_bits_desc_signed(self)
            }

            /// Converts an iterator of bits into a value. The bits should be in ascending order
            /// (least- to most-significant).
            ///
            /// The bits are interpreted as in twos-complement, and the last bit is the sign bit; if
            /// it is `false`, the number is non-negative, and if it is `true`, the number is
            /// negative.
            ///
            /// The function panics if the input represents a number that can't fit in the output
            /// type.
            ///
            /// Let $k$ be `bits.count()`. If $k = 0$ or $b_{k-1}$ is `false`, then
            /// $$
            /// f((b_i)_ {i=0}^{k-1}) = \sum_{i=0}^{k-1}2^i \[b_i\],
            /// $$
            /// where braces denote the Iverson bracket, which converts a bit to 0 or 1.
            ///
            /// If $b_{k-1}$ is `true`, then
            /// $$
            /// f((b_i)_ {i=0}^{k-1}) = \left ( \sum_{i=0}^{k-1}2^i \[b_i\] \right ) - 2^k.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `bits.count()`.
            ///
            /// # Panics
            /// Panics if the bits represent a value that doesn't fit in the output type.
            ///
            /// # Examples
            /// See [here](super::bit_convertible#from_bits_asc).
            #[inline]
            fn from_bits_asc<I: Iterator<Item = bool>>(bits: I) -> $s {
                from_bits_asc_signed::<$u, $s, _>(bits)
            }

            /// Converts an iterator of bits into a value. The bits should be in descending order
            /// (most- to least-significant).
            ///
            /// The bits are interpreted as in twos-complement, and the first bit is the sign bit;
            /// if it is `false`, the number is non-negative, and if it is `true`, the number is
            /// negative.
            ///
            /// The function panics if the input represents a number that can't fit in the output
            /// type.
            ///
            /// If `bits` is empty or $b_0$ is `false`, then
            /// $$
            /// f((b_i)_ {i=0}^{k-1}) = \sum_{i=0}^{k-1}2^{k-i-1} \[b_i\],
            /// $$
            /// where braces denote the Iverson bracket, which converts a bit to 0 or 1.
            ///
            /// If $b_0$ is `true`, then
            /// $$
            /// f((b_i)_ {i=0}^{k-1}) = \left ( \sum_{i=0}^{k-1}2^{k-i-1} \[b_i\] \right ) - 2^k.
            /// $$
            ///
            /// # Examples
            /// See [here](super::bit_convertible#from_bits_desc).
            #[inline]
            fn from_bits_desc<I: Iterator<Item = bool>>(bits: I) -> $s {
                from_bits_desc_signed::<$u, $s, _>(bits)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_bit_convertible_signed);

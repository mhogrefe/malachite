// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::num::logic::traits::BitConvertible;
use itertools::Itertools;

pub fn to_bits_asc_alt<T: BitConvertible>(n: &T) -> Vec<bool> {
    let mut bits = n.to_bits_desc();
    bits.reverse();
    bits
}

pub fn to_bits_desc_alt<T: BitConvertible>(n: &T) -> Vec<bool> {
    let mut bits = n.to_bits_asc();
    bits.reverse();
    bits
}

pub fn from_bits_asc_alt<T: BitConvertible, I: Iterator<Item = bool>>(bits: I) -> T {
    let mut bits = bits.collect_vec();
    bits.reverse();
    T::from_bits_desc(bits.into_iter())
}

pub fn from_bits_desc_alt<T: BitConvertible, I: Iterator<Item = bool>>(bits: I) -> T {
    let mut bits = bits.collect_vec();
    bits.reverse();
    T::from_bits_asc(bits.into_iter())
}

pub fn to_bits_asc_unsigned_naive<T: PrimitiveUnsigned>(n: T) -> Vec<bool> {
    let mut bits = Vec::new();
    for i in 0..n.significant_bits() {
        bits.push(n.get_bit(i));
    }
    bits
}

pub fn to_bits_desc_unsigned_naive<T: PrimitiveUnsigned>(n: T) -> Vec<bool> {
    let mut bits = Vec::new();
    for i in (0..n.significant_bits()).rev() {
        bits.push(n.get_bit(i));
    }
    bits
}

pub fn from_bits_asc_unsigned_naive<T: PrimitiveUnsigned, I: Iterator<Item = bool>>(bits: I) -> T {
    let mut n = T::ZERO;
    for i in bits
        .enumerate()
        .filter_map(|(i, bit)| if bit { Some(u64::exact_from(i)) } else { None })
    {
        n.set_bit(i);
    }
    n
}

pub fn to_bits_asc_signed_naive<T: PrimitiveSigned>(n: T) -> Vec<bool> {
    let mut bits = Vec::new();
    if n == T::ZERO {
        return bits;
    }
    for i in 0..n.significant_bits() {
        bits.push(n.get_bit(i));
    }
    let last_bit = *bits.last().unwrap();
    if last_bit != (n < T::ZERO) {
        bits.push(!last_bit);
    }
    bits
}

pub fn to_bits_desc_signed_naive<T: PrimitiveSigned>(n: T) -> Vec<bool> {
    let mut bits = Vec::new();
    if n == T::ZERO {
        return bits;
    }
    let significant_bits = n.significant_bits();
    let last_bit = n.get_bit(significant_bits - 1);
    if last_bit != (n < T::ZERO) {
        bits.push(!last_bit);
    }
    for i in (0..significant_bits).rev() {
        bits.push(n.get_bit(i));
    }
    bits
}

pub fn from_bits_asc_signed_naive<T: PrimitiveSigned, I: Iterator<Item = bool>>(bits: I) -> T {
    let bits = bits.collect_vec();
    if bits.is_empty() {
        return T::ZERO;
    }
    let mut n;
    if *bits.last().unwrap() {
        n = T::NEGATIVE_ONE;
        for i in
            bits.iter()
                .enumerate()
                .filter_map(|(i, &bit)| if bit { None } else { Some(u64::exact_from(i)) })
        {
            n.clear_bit(i);
        }
    } else {
        n = T::ZERO;
        for i in
            bits.iter()
                .enumerate()
                .filter_map(|(i, &bit)| if bit { Some(u64::exact_from(i)) } else { None })
        {
            n.set_bit(i);
        }
    };
    n
}

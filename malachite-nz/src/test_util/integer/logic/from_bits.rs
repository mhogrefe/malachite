use integer::Integer;
use itertools::Itertools;
use malachite_base::num::basic::traits::{NegativeOne, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;

pub fn from_bits_asc_naive<I: Iterator<Item = bool>>(bits: I) -> Integer {
    let bits = bits.collect_vec();
    if bits.is_empty() {
        return Integer::ZERO;
    }
    let mut n;
    if *bits.last().unwrap() {
        n = Integer::NEGATIVE_ONE;
        for i in
            bits.iter()
                .enumerate()
                .filter_map(|(i, &bit)| if bit { None } else { Some(u64::exact_from(i)) })
        {
            n.clear_bit(i);
        }
    } else {
        n = Integer::ZERO;
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

pub fn from_bits_desc_naive<I: Iterator<Item = bool>>(bits: I) -> Integer {
    let bits = bits.collect_vec();
    if bits.is_empty() {
        return Integer::ZERO;
    }
    let mut n;
    if bits[0] {
        n = Integer::NEGATIVE_ONE;
        for i in bits.iter().rev().enumerate().filter_map(|(i, &bit)| {
            if bit {
                None
            } else {
                Some(u64::exact_from(i))
            }
        }) {
            n.clear_bit(i);
        }
    } else {
        n = Integer::ZERO;
        for i in bits.iter().rev().enumerate().filter_map(|(i, &bit)| {
            if bit {
                Some(u64::exact_from(i))
            } else {
                None
            }
        }) {
            n.set_bit(i);
        }
    };
    n
}

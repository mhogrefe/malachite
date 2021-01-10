use itertools::Itertools;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_nz::natural::Natural;

pub fn from_bits_asc_naive<I: Iterator<Item = bool>>(bits: I) -> Natural {
    let mut n = Natural::ZERO;
    for i in bits.enumerate().filter_map(|(index, bit)| {
        if bit {
            Some(u64::exact_from(index))
        } else {
            None
        }
    }) {
        n.set_bit(i);
    }
    n
}

pub fn from_bits_desc_naive<I: Iterator<Item = bool>>(bits: I) -> Natural {
    let bits = bits.collect_vec();
    let mut n = Natural::ZERO;
    for i in bits.iter().rev().enumerate().filter_map(|(index, &bit)| {
        if bit {
            Some(u64::exact_from(index))
        } else {
            None
        }
    }) {
        n.set_bit(i);
    }
    n
}

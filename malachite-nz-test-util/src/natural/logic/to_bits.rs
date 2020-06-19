use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_nz::natural::Natural;

pub fn to_bits_asc_naive(n: &Natural) -> Vec<bool> {
    let mut bits = Vec::new();
    for i in 0..n.significant_bits() {
        bits.push(n.get_bit(i));
    }
    bits
}

pub fn to_bits_desc_naive(n: &Natural) -> Vec<bool> {
    let mut bits = Vec::new();
    for i in (0..n.significant_bits()).rev() {
        bits.push(n.get_bit(i));
    }
    bits
}

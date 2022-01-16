use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;
use std::cmp::Ordering;

pub fn natural_cmp_normalized_naive(x: &Natural, y: &Natural) -> Ordering {
    let x_bits = x.significant_bits();
    let y_bits = y.significant_bits();
    match x_bits.cmp(&y_bits) {
        Ordering::Equal => x.cmp(y),
        Ordering::Less => (x << (y_bits - x_bits)).cmp(y),
        Ordering::Greater => x.cmp(&(y << (x_bits - y_bits))),
    }
}

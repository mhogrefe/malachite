use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::BitAccess;

pub fn _get_bits_naive<T: BitAccess, U: BitAccess + Zero>(n: &T, start: u64, end: u64) -> U {
    let mut result = U::ZERO;
    for i in start..end {
        if n.get_bit(i) {
            result.set_bit(i - start);
        }
    }
    result
}

pub fn _assign_bits_naive<T: BitAccess, U: BitAccess>(n: &mut T, start: u64, end: u64, bits: &U) {
    for i in start..end {
        n.assign_bit(i, bits.get_bit(i - start));
    }
}

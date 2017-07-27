use integer::Integer;
use natural::{LIMB_BITS, LIMB_BITS_MASK, LOG_LIMB_BITS};
use natural::Natural::{self, Large, Small};

impl Integer {
    /// Determines whether the `index`th bit of an `Integer`, or the coefficient of 2^(`index`) in
    /// its binary expansion, is 0 or 1. `false` means 0, `true` means 1.
    ///
    /// Negative integers are treated as though they are represented in two's complement.
    ///
    /// # Example
    /// ```
    /// use malachite_native::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Integer::from(123).get_bit(2), false);
    /// assert_eq!(Integer::from(123).get_bit(3), true);
    /// assert_eq!(Integer::from(123).get_bit(100), false);
    /// assert_eq!(Integer::from(-123).get_bit(0), true);
    /// assert_eq!(Integer::from(-123).get_bit(1), false);
    /// assert_eq!(Integer::from(-123).get_bit(100), true);
    /// assert_eq!(Integer::from_str("1000000000000").unwrap().get_bit(12), true);
    /// assert_eq!(Integer::from_str("1000000000000").unwrap().get_bit(100), false);
    /// assert_eq!(Integer::from_str("-1000000000000").unwrap().get_bit(12), true);
    /// assert_eq!(Integer::from_str("-1000000000000").unwrap().get_bit(100), true);
    /// ```
    pub fn get_bit(&self, index: u64) -> bool {
        match *self {
            Integer { sign: true, ref abs } => abs.get_bit(index),
            Integer { sign: false, ref abs } => abs.get_bit_neg(index),
        }
    }
}

impl Natural {
    fn get_bit_neg(&self, index: u64) -> bool {
        match *self {
            Small(small) => {
                index >= LIMB_BITS as u64 || ((!small).wrapping_add(1)) & (1 << index) != 0
            }
            Large(ref xs) => {
                let limb_index = (index >> LOG_LIMB_BITS) as usize;
                if limb_index >= xs.len() {
                    // We're indexing into the infinite suffix of 1s
                    return true;
                }
                let limb = if xs.into_iter().take(limb_index).all(|&x| x == 0) {
                    // All limbs below `limb_index` are zero, so we have a carry bit when we take
                    // the two's complement
                    (!xs[limb_index]).wrapping_add(1)
                } else {
                    !xs[limb_index]
                };
                limb & (1 << (index & LIMB_BITS_MASK as u64)) != 0
            }
        }
    }
}

use integer::Integer;
use natural::arithmetic::add_u32::mpn_add_1_in_place;
use natural::{LIMB_BITS, LIMB_BITS_MASK, LOG_LIMB_BITS};
use natural::Natural::{self, Large, Small};

impl Integer {
    /// Sets the `index`th bit of a `Integer`, or the coefficient of 2^(`index`) in its binary
    /// expansion, to 0.
    ///
    /// Time: worst case O(`index`)
    ///
    /// Additional memory: worst case O(`index`)
    ///
    /// # Examples
    /// ```
    /// use malachite_native::integer::Integer;
    ///
    /// let mut x = Integer::from(0x7f);
    /// x.clear_bit(0);
    /// x.clear_bit(1);
    /// x.clear_bit(3);
    /// x.clear_bit(4);
    /// assert_eq!(x.to_string(), "100");
    ///
    /// let mut x = Integer::from(-156);
    /// x.clear_bit(2);
    /// x.clear_bit(5);
    /// x.clear_bit(6);
    /// assert_eq!(x.to_string(), "-256");
    /// ```
    pub fn clear_bit(&mut self, index: u64) {
        match *self {
            Integer {
                sign: true,
                ref mut abs,
            } => abs.clear_bit(index),
            Integer {
                sign: false,
                ref mut abs,
            } => abs.clear_bit_neg(index),
        }
    }
}

impl Natural {
    // self cannot be zero
    fn clear_bit_neg(&mut self, index: u64) {
        mutate_with_possible_promotion!(
            self,
            small,
            limbs,
            {
                if index < LIMB_BITS.into() {
                    Some(((*small - 1) | (1 << index)).wrapping_add(1))
                } else {
                    None
                }
            },
            {
                let limb_index = (index >> LOG_LIMB_BITS) as usize;
                let mask = 1 << ((index & u64::from(LIMB_BITS_MASK)) as u32);
                if limb_index < limbs.len() {
                    let mut zero_bound = 0;
                    // No index upper bound on this loop; we're sure there's a nonzero limb sooner
                    // or later.
                    while limbs[zero_bound] == 0 {
                        zero_bound += 1;
                    }
                    if limb_index > zero_bound {
                        limbs[limb_index] |= mask;
                    } else if limb_index == zero_bound {
                        let dlimb = ((limbs[limb_index] - 1) | mask).wrapping_add(1);
                        limbs[limb_index] = dlimb;
                        if dlimb == 0 && mpn_add_1_in_place(&mut limbs[limb_index + 1..], 1) {
                            limbs.push(1);
                        }
                    }
                } else {
                    limbs.resize(limb_index, 0);
                    limbs.push(mask);
                }
            }
        );
    }
}

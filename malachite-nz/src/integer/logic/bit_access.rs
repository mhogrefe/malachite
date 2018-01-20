use integer::Integer;
use malachite_base::num::BitAccess;
use natural::{LIMB_BITS, LIMB_BITS_MASK, LOG_LIMB_BITS};
use natural::arithmetic::add_u32::mpn_add_1_in_place;
use natural::arithmetic::sub_u32::mpn_sub_1_in_place;
use natural::Natural::{self, Large, Small};

/// Provides functions for accessing and modifying the `index`th bit of a `Natural`, or the
/// coefficient of 2^<pow>`index`</pow> in its binary expansion.
///
/// Negative integers are treated as though they are represented in two's complement.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::BitAccess;
/// use malachite_base::traits::{NegativeOne, Zero};
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::ZERO;
///     x.assign_bit(2, true);
///     x.assign_bit(5, true);
///     x.assign_bit(6, true);
///     assert_eq!(x.to_string(), "100");
///     x.assign_bit(2, false);
///     x.assign_bit(5, false);
///     x.assign_bit(6, false);
///     assert_eq!(x.to_string(), "0");
///
///     let mut x = Integer::from(-0x100);
///     x.assign_bit(2, true);
///     x.assign_bit(5, true);
///     x.assign_bit(6, true);
///     assert_eq!(x.to_string(), "-156");
///     x.assign_bit(2, false);
///     x.assign_bit(5, false);
///     x.assign_bit(6, false);
///     assert_eq!(x.to_string(), "-256");
///
///     let mut x = Integer::ZERO;
///     x.flip_bit(10);
///     assert_eq!(x.to_string(), "1024");
///     x.flip_bit(10);
///     assert_eq!(x.to_string(), "0");
///
///     let mut x = Integer::NEGATIVE_ONE;
///     x.flip_bit(10);
///     assert_eq!(x.to_string(), "-1025");
///     x.flip_bit(10);
///     assert_eq!(x.to_string(), "-1");
/// }
/// ```
impl BitAccess for Integer {
    /// Determines whether the `index`th bit of an `Integer`, or the coefficient of
    /// 2<pow>`index`</pow> in its binary expansion, is 0 or 1. `false` means 0, `true` means 1.
    ///
    /// Negative integers are treated as though they are represented in two's complement.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::BitAccess;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::from(123).get_bit(2), false);
    ///     assert_eq!(Integer::from(123).get_bit(3), true);
    ///     assert_eq!(Integer::from(123).get_bit(100), false);
    ///     assert_eq!(Integer::from(-123).get_bit(0), true);
    ///     assert_eq!(Integer::from(-123).get_bit(1), false);
    ///     assert_eq!(Integer::from(-123).get_bit(100), true);
    ///     assert_eq!(Integer::trillion().get_bit(12), true);
    ///     assert_eq!(Integer::trillion().get_bit(100), false);
    ///     assert_eq!((-Integer::trillion()).get_bit(12), true);
    ///     assert_eq!((-Integer::trillion()).get_bit(100), true);
    /// }
    /// ```
    fn get_bit(&self, index: u64) -> bool {
        match *self {
            Integer {
                sign: true,
                ref abs,
            } => abs.get_bit(index),
            Integer {
                sign: false,
                ref abs,
            } => abs.get_bit_neg(index),
        }
    }

    /// Sets the `index`th bit of a `Integer`, or the coefficient of 2<pow>`index`</pow> in its
    /// binary expansion, to 1.
    ///
    /// Negative integers are treated as though they are represented in two's complement.
    ///
    /// Time: worst case O(`index`)
    ///
    /// Additional memory: worst case O(`index`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::BitAccess;
    /// use malachite_base::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     let mut x = Integer::ZERO;
    ///     x.set_bit(2);
    ///     x.set_bit(5);
    ///     x.set_bit(6);
    ///     assert_eq!(x.to_string(), "100");
    ///
    ///     let mut x = Integer::from(-0x100);
    ///     x.set_bit(2);
    ///     x.set_bit(5);
    ///     x.set_bit(6);
    ///     assert_eq!(x.to_string(), "-156");
    /// }
    /// ```
    fn set_bit(&mut self, index: u64) {
        match *self {
            Integer {
                sign: true,
                ref mut abs,
            } => abs.set_bit(index),
            Integer {
                sign: false,
                ref mut abs,
            } => abs.set_bit_neg(index),
        }
    }

    /// Sets the `index`th bit of a `Integer`, or the coefficient of 2<pow>`index`</pow> in its
    /// binary expansion, to 0.
    ///
    /// Negative integers are treated as though they are represented in two's complement.
    ///
    /// Time: worst case O(`index`)
    ///
    /// Additional memory: worst case O(`index`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::BitAccess;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     let mut x = Integer::from(0x7f);
    ///     x.clear_bit(0);
    ///     x.clear_bit(1);
    ///     x.clear_bit(3);
    ///     x.clear_bit(4);
    ///     assert_eq!(x.to_string(), "100");
    ///
    ///     let mut x = Integer::from(-156);
    ///     x.clear_bit(2);
    ///     x.clear_bit(5);
    ///     x.clear_bit(6);
    ///     assert_eq!(x.to_string(), "-256");
    /// }
    /// ```
    fn clear_bit(&mut self, index: u64) {
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
    fn get_bit_neg(&self, index: u64) -> bool {
        match *self {
            Small(small) => index >= LIMB_BITS.into() || (!small).wrapping_add(1).get_bit(index),
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
                limb.get_bit(index & u64::from(LIMB_BITS_MASK))
            }
        }
    }

    // self cannot be zero
    fn set_bit_neg(&mut self, index: u64) {
        match *self {
            Small(ref mut small) => {
                if index < LIMB_BITS.into() {
                    *small -= 1;
                    small.clear_bit(index);
                    *small += 1;
                }
                return;
            }
            Large(ref mut limbs) => {
                let limb_index = (index >> LOG_LIMB_BITS) as usize;
                if limb_index >= limbs.len() {
                    return;
                }
                let reduced_index = index & u64::from(LIMB_BITS_MASK);
                let mut zero_bound = 0;
                // No index upper bound on this loop; we're sure there's a nonzero limb sooner or
                // later.
                while limbs[zero_bound] == 0 {
                    zero_bound += 1;
                }
                if limb_index > zero_bound {
                    limbs[limb_index].clear_bit(reduced_index);
                } else if limb_index == zero_bound {
                    let limb_ref = &mut limbs[limb_index];
                    *limb_ref -= 1;
                    limb_ref.clear_bit(reduced_index);
                    *limb_ref += 1;
                } else {
                    mpn_sub_1_in_place(&mut limbs[limb_index..], 1 << (reduced_index as u32));
                }
            }
        }
        self.trim();
    }

    // self cannot be zero
    fn clear_bit_neg(&mut self, index: u64) {
        mutate_with_possible_promotion!(
            self,
            small,
            limbs,
            {
                if index < LIMB_BITS.into() {
                    *small -= 1;
                    small.set_bit(index);
                    Some(small.wrapping_add(1))
                } else {
                    None
                }
            },
            {
                let limb_index = (index >> LOG_LIMB_BITS) as usize;
                let reduced_index = index & u64::from(LIMB_BITS_MASK);
                if limb_index < limbs.len() {
                    let mut zero_bound = 0;
                    // No index upper bound on this loop; we're sure there's a nonzero limb sooner
                    // or later.
                    while limbs[zero_bound] == 0 {
                        zero_bound += 1;
                    }
                    if limb_index > zero_bound {
                        limbs[limb_index].set_bit(reduced_index);
                    } else if limb_index == zero_bound {
                        let mut dlimb = limbs[limb_index] - 1;
                        dlimb.set_bit(reduced_index);
                        dlimb = dlimb.wrapping_add(1);
                        limbs[limb_index] = dlimb;
                        if dlimb == 0 && mpn_add_1_in_place(&mut limbs[limb_index + 1..], 1) {
                            limbs.push(1);
                        }
                    }
                } else {
                    limbs.resize(limb_index, 0);
                    limbs.push(1 << (reduced_index as u32));
                }
            }
        );
    }
}

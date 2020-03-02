use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;

use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, gets a bit of
/// the `Natural` at a specified index. Sufficiently high indices will return `false`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// This is mpz_tstbit from mpz/tstbit.c, GMP 6.1.2, where the input is non-negative.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::bit_access::limbs_get_bit;
///
/// assert_eq!(limbs_get_bit(&[0, 0b1011], 0), false);
/// assert_eq!(limbs_get_bit(&[0, 0b1011], 32), true);
/// assert_eq!(limbs_get_bit(&[0, 0b1011], 33), true);
/// assert_eq!(limbs_get_bit(&[0, 0b1011], 34), false);
/// assert_eq!(limbs_get_bit(&[0, 0b1011], 35), true);
/// assert_eq!(limbs_get_bit(&[0, 0b1011], 100), false);
/// ```
pub fn limbs_get_bit(limbs: &[Limb], index: u64) -> bool {
    limbs
        .get(usize::exact_from(index >> Limb::LOG_WIDTH))
        .map_or(false, |limb| limb.get_bit(index & Limb::WIDTH_MASK))
}

fn limbs_set_bit_helper(limbs: &mut [Limb], index: u64, limb_index: usize) {
    limbs[limb_index].set_bit(index & Limb::WIDTH_MASK);
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, sets a bit of
/// the `Natural` at a specified index to `true`. Indices that are outside the bounds of the slice
/// will cause a panic.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// This is mpz_setbit from mpz/setbit.c, GMP 6.1.2, where d is non-negative and bit_idx small
/// enough that no additional memory needs to be given to d.
///
/// # Panics
/// Panics if `index` >= `limbs.len()` * 32.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::bit_access::limbs_slice_set_bit;
///
/// let mut limbs = &mut [0, 1];
/// limbs_slice_set_bit(limbs, 0);
/// assert_eq!(limbs, &[1, 1]);
/// limbs_slice_set_bit(limbs, 1);
/// assert_eq!(limbs, &[3, 1]);
/// limbs_slice_set_bit(limbs, 33);
/// assert_eq!(limbs, &[3, 3]);
/// ```
pub fn limbs_slice_set_bit(limbs: &mut [Limb], index: u64) {
    limbs_set_bit_helper(limbs, index, usize::exact_from(index >> Limb::LOG_WIDTH));
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, sets a bit of
/// the `Natural` at a specified index to `true`. Sufficiently high indices will increase the length
/// of the limbs vector.
///
/// Time: worst case O(`index`)
///
/// Additional memory: worst case O(`index`)
///
/// This is mpz_setbit from mpz/setbit.c, GMP 6.1.2, where d is non-negative.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::bit_access::limbs_vec_set_bit;
///
/// let mut limbs = vec![0, 1];
/// limbs_vec_set_bit(&mut limbs, 0);
/// assert_eq!(limbs, &[1, 1]);
/// limbs_vec_set_bit(&mut limbs, 1);
/// assert_eq!(limbs, &[3, 1]);
/// limbs_vec_set_bit(&mut limbs, 33);
/// assert_eq!(limbs, &[3, 3]);
/// limbs_vec_set_bit(&mut limbs, 128);
/// assert_eq!(limbs, &[3, 3, 0, 0, 1]);
/// ```
pub fn limbs_vec_set_bit(limbs: &mut Vec<Limb>, index: u64) {
    let limb_index = usize::exact_from(index >> Limb::LOG_WIDTH);
    if limb_index >= limbs.len() {
        limbs.resize(limb_index + 1, 0);
    }
    limbs_set_bit_helper(limbs, index, limb_index);
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, sets a bit of
/// the `Natural` at a specified index to `false`. Indices that are outside the bounds of the slice
/// will result in no action being taken, since there are infinitely many leading zeros.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// This is mpz_clrbit from mpz/clrbit.c, GMP 6.1.2, where d is non-negative.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::bit_access::limbs_clear_bit;
///
/// let mut limbs = &mut [3, 3];
/// limbs_clear_bit(limbs, 33);
/// assert_eq!(limbs, &[3, 1]);
/// limbs_clear_bit(limbs, 1);
/// assert_eq!(limbs, &[1, 1]);
/// ```
pub fn limbs_clear_bit(limbs: &mut [Limb], index: u64) {
    let limb_index = usize::exact_from(index >> Limb::LOG_WIDTH);
    if limb_index < limbs.len() {
        limbs[limb_index].clear_bit(index & Limb::WIDTH_MASK);
    }
}

/// Provides functions for accessing and modifying the `index`th bit of a `Natural`, or the
/// coefficient of 2^<sup>`index`</sup> in its binary expansion.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::logic::traits::BitAccess;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// let mut x = Natural::ZERO;
/// x.assign_bit(2, true);
/// x.assign_bit(5, true);
/// x.assign_bit(6, true);
/// assert_eq!(x.to_string(), "100");
/// x.assign_bit(2, false);
/// x.assign_bit(5, false);
/// x.assign_bit(6, false);
/// assert_eq!(x.to_string(), "0");
///
/// let mut x = Natural::ZERO;
/// x.flip_bit(10);
/// assert_eq!(x.to_string(), "1024");
/// x.flip_bit(10);
/// assert_eq!(x.to_string(), "0");
/// ```
impl BitAccess for Natural {
    /// Determines whether the `index`th bit of a `Natural`, or the coefficient of
    /// 2<sup>`index`</sup> in its binary expansion, is 0 or 1. `false` means 0, `true` means 1.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitAccess;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(123u32).get_bit(2), false);
    /// assert_eq!(Natural::from(123u32).get_bit(3), true);
    /// assert_eq!(Natural::from(123u32).get_bit(100), false);
    /// assert_eq!(Natural::trillion().get_bit(12), true);
    /// assert_eq!(Natural::trillion().get_bit(100), false);
    /// ```
    fn get_bit(&self, index: u64) -> bool {
        match *self {
            Natural(Small(small)) => small.get_bit(index),
            Natural(Large(ref limbs)) => limbs_get_bit(limbs, index),
        }
    }

    /// Sets the `index`th bit of a `Natural`, or the coefficient of 2<sup>`index`</sup> in its
    /// binary expansion, to 1.
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
    /// use malachite_base::num::logic::traits::BitAccess;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::ZERO;
    /// x.set_bit(2);
    /// x.set_bit(5);
    /// x.set_bit(6);
    /// assert_eq!(x.to_string(), "100");
    /// ```
    fn set_bit(&mut self, index: u64) {
        mutate_with_possible_promotion!(
            self,
            small,
            limbs,
            {
                if index < Limb::WIDTH {
                    let mut modified = *small;
                    modified.set_bit(index);
                    Some(modified)
                } else {
                    None
                }
            },
            limbs_vec_set_bit(limbs, index)
        );
    }

    /// Sets the `index`th bit of a `Natural`, or the coefficient of 2<sup>`index`</sup> in its
    /// binary expansion, to 0.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `index`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::logic::traits::BitAccess;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(0x7fu32);
    /// x.clear_bit(0);
    /// x.clear_bit(1);
    /// x.clear_bit(3);
    /// x.clear_bit(4);
    /// assert_eq!(x.to_string(), "100");
    /// ```
    fn clear_bit(&mut self, index: u64) {
        match *self {
            Natural(Small(ref mut small)) => {
                small.clear_bit(index);
                return;
            }
            Natural(Large(ref mut limbs)) => limbs_clear_bit(limbs, index),
        }
        self.trim();
    }
}

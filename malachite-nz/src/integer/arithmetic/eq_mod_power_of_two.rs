use integer::Integer;
use malachite_base::misc::Max;
use malachite_base::num::{EqModPowerOfTwo, PrimitiveInteger};
use natural::Natural::{self, Large, Small};
use platform::Limb;

fn limbs_eq_mod_power_of_two_neg_pos_greater(xs: &[Limb], ys: &[Limb], pow: u64) -> bool {
    let xs_len = xs.len();
    let i = (pow >> Limb::LOG_WIDTH) as usize;
    let small_pow = pow & u64::from(Limb::WIDTH_MASK);
    if i > xs_len || i == xs_len && small_pow != 0 {
        false
    } else {
        let ys_len = ys.len();
        let mut y_nonzero_seen = false;
        for j in 0..i {
            let y = if j >= ys_len {
                Limb::MAX
            } else if y_nonzero_seen {
                !ys[j]
            } else if ys[j] == 0 {
                0
            } else {
                y_nonzero_seen = true;
                ys[j].wrapping_neg()
            };
            if xs[j] != y {
                return false;
            }
        }
        if small_pow == 0 {
            true
        } else {
            // i < xs_len
            let y = if i >= ys_len {
                Limb::MAX
            } else if y_nonzero_seen {
                !ys[i]
            } else {
                ys[i].wrapping_neg()
            };
            xs[i].eq_mod_power_of_two(y, small_pow)
        }
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, returns
/// whether the first `Natural` and the negative of the second natural (equivalently, the negative
/// of the first `Natural` and the second `Natural`) are equivalent mod two to the power of `pow`;
/// that is, whether their `pow` least-significant bits are equal.
///
/// This function assumes that neither slice is empty and their last elements are nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = min(pow, max(`xs.len()`, `ys.len()`))
///
/// # Example
/// ```
/// use malachite_nz::integer::arithmetic::eq_mod_power_of_two::limbs_eq_mod_power_of_two_neg_pos;
///
/// assert_eq!(
///     limbs_eq_mod_power_of_two_neg_pos(&[0xabcd_abcd, 0x1234_1234], &[0x5432_5433, 0xedcb_edcb],
///     64), true);
/// assert_eq!(
///     limbs_eq_mod_power_of_two_neg_pos(&[0xabcd_abcd, 0x1234_1234], &[0x0000_0000, 0xedcb_edcb],
///     64), false);
/// assert_eq!(
///     limbs_eq_mod_power_of_two_neg_pos(&[0xabcd_abcd, 0x1234_1234], &[0x5432_5433, 0xedcb_edcb],
///     65), false);
/// assert_eq!(
///     limbs_eq_mod_power_of_two_neg_pos(&[0xabcd_abcd, 0x1234_1234], &[0x5432_5433, 0xedcb_edcb],
///     128), false);
/// ```
pub fn limbs_eq_mod_power_of_two_neg_pos(xs: &[Limb], ys: &[Limb], pow: u64) -> bool {
    if xs.len() >= ys.len() {
        limbs_eq_mod_power_of_two_neg_pos_greater(xs, ys, pow)
    } else {
        limbs_eq_mod_power_of_two_neg_pos_greater(ys, xs, pow)
    }
}

impl<'a, 'b> EqModPowerOfTwo<&'b Integer> for &'a Integer {
    /// Returns whether two `Integer`s are equivalent mod two to the power of `pow`; that is,
    /// whether their `pow` least-significant bits are equal.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = min(`pow`, max(`self.significant_bits()`, `other.significant_bits()`))
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::{EqModPowerOfTwo, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!((&Integer::ZERO).eq_mod_power_of_two(&Integer::from(-256), 8), true);
    ///     assert_eq!((&Integer::from(-0b1101)).eq_mod_power_of_two(&Integer::from(0b11011),
    ///         3), true);
    ///     assert_eq!((&Integer::from(-0b1101)).eq_mod_power_of_two(&Integer::from(0b11011),
    ///         4), false);
    /// }
    /// ```
    fn eq_mod_power_of_two(self, other: &'b Integer, pow: u64) -> bool {
        if self.sign == other.sign {
            self.abs.eq_mod_power_of_two(&other.abs, pow)
        } else {
            self.abs.eq_mod_power_of_two_neg_pos(&other.abs, pow)
        }
    }
}

impl Natural {
    fn eq_mod_power_of_two_neg_pos(&self, other: &Natural, pow: u64) -> bool {
        match (self, other) {
            (_, &Small(y)) => self.eq_mod_power_of_two_neg_limb(y, pow),
            (&Small(x), _) => other.eq_mod_power_of_two_neg_limb(x, pow),
            (&Large(ref xs), &Large(ref ys)) => limbs_eq_mod_power_of_two_neg_pos(xs, ys, pow),
        }
    }
}

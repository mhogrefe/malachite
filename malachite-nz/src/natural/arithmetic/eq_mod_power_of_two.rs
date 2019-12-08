use std::cmp::Ordering;

use malachite_base::num::arithmetic::traits::EqModPowerOfTwo;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::{CheckedFrom, WrappingFrom};

use natural::arithmetic::divisible_by_power_of_two::limbs_divisible_by_power_of_two;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

// xs.len() == ys.len()
fn limbs_eq_mod_power_of_two_same_length(xs: &[Limb], ys: &[Limb], pow: u64) -> bool {
    let i = usize::checked_from(pow >> Limb::LOG_WIDTH).unwrap();
    let len = xs.len();
    if i >= len {
        xs == ys
    } else {
        xs[..i] == ys[..i] && xs[i].eq_mod_power_of_two(ys[i], pow & u64::from(Limb::WIDTH_MASK))
    }
}

// xs.len() > ys.len()
fn limbs_eq_mod_power_of_two_greater(xs: &[Limb], ys: &[Limb], pow: u64) -> bool {
    let i = usize::checked_from(pow >> Limb::LOG_WIDTH).unwrap();
    let xs_len = xs.len();
    let ys_len = ys.len();
    if i >= xs_len {
        false
    } else if i >= ys_len {
        &xs[..ys_len] == ys
            && limbs_divisible_by_power_of_two(
                &xs[ys_len..],
                pow - u64::from(Limb::WIDTH) * u64::wrapping_from(ys_len),
            )
    } else {
        xs[..i] == ys[..i] && xs[i].eq_mod_power_of_two(ys[i], pow & u64::from(Limb::WIDTH_MASK))
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, returns
/// whether the `Natural`s are equivalent mod two to the power of `pow`; that is, whether their
/// `pow` least-significant bits are equal.
///
/// This function assumes that neither slice is empty and their last elements are nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = min(pow, `xs.len()`, `ys.len()`)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::eq_mod_power_of_two::limbs_eq_mod_power_of_two;
///
/// assert_eq!(limbs_eq_mod_power_of_two(&[0b1111011, 0b111001000], &[0b101011], 4), true);
/// assert_eq!(limbs_eq_mod_power_of_two(&[0b1111011, 0b111001000], &[0b101011], 5), false);
/// assert_eq!(limbs_eq_mod_power_of_two(&[0b1111011, 0b111001000], &[0b1111011, 0b111101000], 37),
///         true);
/// assert_eq!(limbs_eq_mod_power_of_two(&[0b1111011, 0b111001000], &[0b1111011, 0b111101000], 38),
///         false);
/// assert_eq!(limbs_eq_mod_power_of_two(&[0b1111011, 0b111001000], &[0b1111011, 0b111101000], 100),
///         false);
/// ```
///
/// This is mpz_congruent_2exp_p from mpz/cong_2exp.c, where a and c are non-negative.
pub fn limbs_eq_mod_power_of_two(xs: &[Limb], ys: &[Limb], pow: u64) -> bool {
    match xs.len().cmp(&ys.len()) {
        Ordering::Equal => limbs_eq_mod_power_of_two_same_length(xs, ys, pow),
        Ordering::Less => limbs_eq_mod_power_of_two_greater(ys, xs, pow),
        Ordering::Greater => limbs_eq_mod_power_of_two_greater(xs, ys, pow),
    }
}

impl<'a, 'b> EqModPowerOfTwo<&'b Natural> for &'a Natural {
    /// Returns whether two `Natural`s are equivalent mod two to the power of `pow`; that is,
    /// whether their `pow` least-significant bits are equal.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = min(`pow`, `self.significant_bits()`, `other.significant_bits()`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::EqModPowerOfTwo;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!((&Natural::ZERO).eq_mod_power_of_two(&Natural::from(256u32), 8), true);
    ///     assert_eq!((&Natural::from(0b1101u32)).eq_mod_power_of_two(&Natural::from(0b10101u32),
    ///         3), true);
    ///     assert_eq!((&Natural::from(0b1101u32)).eq_mod_power_of_two(&Natural::from(0b10101u32),
    ///         4), false);
    /// }
    /// ```
    fn eq_mod_power_of_two(self, other: &'b Natural, pow: u64) -> bool {
        match (self, other) {
            (_, &Natural(Small(y))) => self.eq_mod_power_of_two(y, pow),
            (&Natural(Small(x)), _) => other.eq_mod_power_of_two(x, pow),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                limbs_eq_mod_power_of_two(xs, ys, pow)
            }
        }
    }
}

use malachite_base::num::arithmetic::traits::EqModPowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use natural::arithmetic::divisible_by_power_of_2::limbs_divisible_by_power_of_2;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;
use std::cmp::Ordering;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns
/// whether the `Natural` is equivalent to a limb mod two to the power of `pow`; that is, whether
/// the `pow` least-significant bits of the `Natural` and the limb are equal.
///
/// This function assumes that `xs` has length at least 2 and the last (most significant) limb is
/// nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
#[doc(hidden)]
pub fn limbs_eq_limb_mod_power_of_2(xs: &[Limb], y: Limb, pow: u64) -> bool {
    let i = usize::exact_from(pow >> Limb::LOG_WIDTH);
    if i >= xs.len() {
        false
    } else if i == 0 {
        xs[0].eq_mod_power_of_2(y, pow)
    } else {
        let (xs_head, xs_tail) = xs.split_first().unwrap();
        *xs_head == y && limbs_divisible_by_power_of_2(xs_tail, pow - Limb::WIDTH)
    }
}

// xs.len() == ys.len()
fn limbs_eq_mod_power_of_2_same_length(xs: &[Limb], ys: &[Limb], pow: u64) -> bool {
    let i = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let len = xs.len();
    if i >= len {
        xs == ys
    } else {
        let (xs_last, xs_init) = xs[..i + 1].split_last().unwrap();
        let (ys_last, ys_init) = ys[..i + 1].split_last().unwrap();
        xs_init == ys_init && xs_last.eq_mod_power_of_2(*ys_last, pow & Limb::WIDTH_MASK)
    }
}

// xs.len() > ys.len()
fn limbs_eq_mod_power_of_2_greater(xs: &[Limb], ys: &[Limb], pow: u64) -> bool {
    let i = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let xs_len = xs.len();
    let ys_len = ys.len();
    if i >= xs_len {
        false
    } else if i >= ys_len {
        let (xs_lo, xs_hi) = xs.split_at(ys_len);
        xs_lo == ys
            && limbs_divisible_by_power_of_2(xs_hi, pow - Limb::WIDTH * u64::wrapping_from(ys_len))
    } else {
        let (xs_last, xs_init) = xs[..i + 1].split_last().unwrap();
        let (ys_last, ys_init) = ys[..i + 1].split_last().unwrap();
        xs_init == ys_init && xs_last.eq_mod_power_of_2(*ys_last, pow & Limb::WIDTH_MASK)
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
/// This is mpz_congruent_2exp_p from mpz/cong_2exp.c, GMP 6.2.1, where a and c are non-negative.
#[doc(hidden)]
pub fn limbs_eq_mod_power_of_2(xs: &[Limb], ys: &[Limb], pow: u64) -> bool {
    match xs.len().cmp(&ys.len()) {
        Ordering::Equal => limbs_eq_mod_power_of_2_same_length(xs, ys, pow),
        Ordering::Less => limbs_eq_mod_power_of_2_greater(ys, xs, pow),
        Ordering::Greater => limbs_eq_mod_power_of_2_greater(xs, ys, pow),
    }
}

impl Natural {
    fn eq_mod_power_of_2_limb(&self, other: Limb, pow: u64) -> bool {
        match *self {
            Natural(Small(small)) => small.eq_mod_power_of_2(other, pow),
            Natural(Large(ref limbs)) => limbs_eq_limb_mod_power_of_2(limbs, other, pow),
        }
    }
}

impl<'a, 'b> EqModPowerOf2<&'b Natural> for &'a Natural {
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
    /// use malachite_base::num::arithmetic::traits::EqModPowerOf2;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::ZERO).eq_mod_power_of_2(&Natural::from(256u32), 8), true);
    /// assert_eq!(
    ///     (&Natural::from(0b1101u32)).eq_mod_power_of_2(&Natural::from(0b10101u32), 3),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Natural::from(0b1101u32)).eq_mod_power_of_2(&Natural::from(0b10101u32), 4),
    ///     false
    /// );
    /// ```
    fn eq_mod_power_of_2(self, other: &'b Natural, pow: u64) -> bool {
        match (self, other) {
            (_, &Natural(Small(y))) => self.eq_mod_power_of_2_limb(y, pow),
            (&Natural(Small(x)), _) => other.eq_mod_power_of_2_limb(x, pow),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                limbs_eq_mod_power_of_2(xs, ys, pow)
            }
        }
    }
}

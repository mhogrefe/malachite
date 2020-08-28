use integer::conversion::to_twos_complement_limbs::limbs_twos_complement_in_place;
use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwoNeg, ModPowerOfTwoNegAssign, ModPowerOfTwoSub, ModPowerOfTwoSubAssign, ShrRound,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use natural::arithmetic::mod_power_of_two::{
    limbs_neg_mod_power_of_two, limbs_neg_mod_power_of_two_in_place,
    limbs_slice_mod_power_of_two_in_place,
};
use natural::arithmetic::mod_power_of_two_add::limbs_vec_mod_power_of_two_add_limb_in_place;
use natural::arithmetic::sub::{
    limbs_sub_in_place_left, limbs_sub_limb, limbs_sub_limb_in_place,
    limbs_sub_same_length_in_place_right, limbs_vec_sub_in_place_right,
};
use natural::logic::low_mask::limbs_low_mask;
use natural::logic::not::limbs_not_in_place;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;
use std::mem::swap;

fn extend_with_ones(xs: &mut Vec<Limb>, pow: u64) {
    xs.resize(
        usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, RoundingMode::Ceiling)),
        Limb::MAX,
    );
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, subtracts the
/// `Natural` from a `Limb`, mod 2<sup>`pow`</sup>. Assumes the input is already reduced mod
/// 2<sup>`pow`</sup>.
///
/// Time: worst case O(`pow`)
///
/// Additional memory: worst case O(`pow`)
///
/// # Panics
/// Panics if `pow` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_two_sub::*;
///
/// assert_eq!(limbs_mod_power_of_two_limb_sub_limbs(3, &[2], 4), &[1]);
/// assert_eq!(limbs_mod_power_of_two_limb_sub_limbs(3, &[1, 2, 3], 70), &[2, u32::MAX - 1, 60]);
/// ```
pub fn limbs_mod_power_of_two_limb_sub_limbs(x: Limb, ys: &[Limb], pow: u64) -> Vec<Limb> {
    let mut diff = limbs_neg_mod_power_of_two(ys, pow);
    limbs_vec_mod_power_of_two_add_limb_in_place(&mut diff, x, pow);
    diff
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, subtracts the
/// `Natural` from a `Limb`, mod 2<sup>`pow`</sup>, and writes the limbs of the difference to the
/// input slice. Assumes the input is already reduced mod 2<sup>`pow`</sup>.
///
/// Time: worst case O(`pow`)
///
/// Additional memory: worst case O(`pow`)
///
/// # Panics
/// Panics if `pow` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_two_sub::*;
///
/// let mut ys = vec![2];
/// limbs_mod_power_of_two_limb_sub_limbs_in_place(3, &mut ys, 4);
/// assert_eq!(ys, &[1]);
///
/// let mut ys = vec![1, 2, 3];
/// limbs_mod_power_of_two_limb_sub_limbs_in_place(3, &mut ys, 70);
/// assert_eq!(ys, &[2, u32::MAX - 1, 60]);
/// ```
pub fn limbs_mod_power_of_two_limb_sub_limbs_in_place(x: Limb, ys: &mut Vec<Limb>, pow: u64) {
    limbs_neg_mod_power_of_two_in_place(ys, pow);
    limbs_vec_mod_power_of_two_add_limb_in_place(ys, x, pow);
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s,
/// subtracts the second `Natural` from the first, mod 2<sup>`pow`</sup>, and returns a `Vec` of the
/// limbs of the difference. Assumes the inputs are already reduced mod 2<sup>`pow`</sup>.
///
/// Time: worst case O(`pow`)
///
/// Additional memory: worst case O(`pow`)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_two_sub::limbs_mod_power_of_two_sub;
///
/// assert_eq!(limbs_mod_power_of_two_sub(&[1, 2, 3], &[6, 7], 100), &[4294967291, 4294967290, 2]);
/// assert_eq!(limbs_mod_power_of_two_sub(&[6, 7], &[1, 2, 3], 100), &[5, 5, 4294967293, 15]);
/// ```
pub fn limbs_mod_power_of_two_sub(xs: &[Limb], ys: &[Limb], pow: u64) -> Vec<Limb> {
    let ys_len = ys.len();
    let mut out_limbs = xs.to_vec();
    if ys_len > xs.len() {
        out_limbs.resize(ys_len, 0);
    }
    if limbs_sub_in_place_left(&mut out_limbs, ys) {
        extend_with_ones(&mut out_limbs, pow);
        limbs_slice_mod_power_of_two_in_place(&mut out_limbs, pow);
    }
    out_limbs
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s,
/// subtracts the second `Natural` from the first, mod 2<sup>`pow`</sup>, and writes the limbs of
/// the difference to the first (left) slice. Assumes the inputs are already reduced mod
/// 2<sup>`pow`</sup>.
///
/// Time: worst case O(`pow`)
///
/// Additional memory: worst case O(`pow`)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_two_sub::*;
///
/// let mut xs = vec![1, 2, 3];
/// limbs_mod_power_of_two_sub_in_place_left(&mut xs, &[6, 7], 100);
/// assert_eq!(xs, &[4294967291, 4294967290, 2]);
///
/// let mut xs = vec![6, 7];
/// limbs_mod_power_of_two_sub_in_place_left(&mut xs, &[1, 2, 3], 100);
/// assert_eq!(xs, &[5, 5, 4294967293, 15]);
/// ```
pub fn limbs_mod_power_of_two_sub_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb], pow: u64) {
    let ys_len = ys.len();
    if ys_len > xs.len() {
        xs.resize(ys_len, 0);
    }
    if limbs_sub_in_place_left(xs, ys) {
        extend_with_ones(xs, pow);
        limbs_slice_mod_power_of_two_in_place(xs, pow);
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s,
/// subtracts the second `Natural` from the first, mod 2<sup>`pow`</sup>, and writes the limbs of
/// the difference to the second (right) slice. Assumes the inputs are already reduced mod
/// 2<sup>`pow`</sup>.
///
/// Neither input slice may have trailing zeros.
///
/// Time: worst case O(`pow`)
///
/// Additional memory: worst case O(`pow`)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_two_sub::*;
///
/// let mut ys = vec![6, 7];
/// limbs_mod_power_of_two_sub_in_place_right(&[1, 2, 3], &mut ys, 100);
/// assert_eq!(ys, &[4294967291, 4294967290, 2]);
///
/// let mut ys = vec![1, 2, 3];
/// limbs_mod_power_of_two_sub_in_place_right(&[6, 7], &mut ys, 100);
/// assert_eq!(ys, &[5, 5, 4294967293, 15]);
/// ```
pub fn limbs_mod_power_of_two_sub_in_place_right(xs: &[Limb], ys: &mut Vec<Limb>, pow: u64) {
    let xs_len = xs.len();
    if xs_len >= ys.len() {
        if limbs_vec_sub_in_place_right(xs, ys) {
            extend_with_ones(ys, pow);
            limbs_slice_mod_power_of_two_in_place(ys, pow);
        }
    } else {
        let (ys_lo, ys_hi) = ys.split_at_mut(xs_len);
        if limbs_sub_same_length_in_place_right(xs, ys_lo) {
            limbs_not_in_place(ys_hi);
        } else {
            limbs_twos_complement_in_place(ys_hi);
        }
        extend_with_ones(ys, pow);
        limbs_slice_mod_power_of_two_in_place(ys, pow);
    }
}

/// Interpreting two `Vec`s of `Limb`s as the limbs (in ascending order) of two `Natural`s,
/// subtracts the second `Natural` from the first, mod 2<sup>`pow`</sup>, and writes the limbs of
/// the difference to to the longer slice (or the first one, if they are equally long). Returns a
/// `bool` which is `false` when the output is to the first `Vec` and `true` when it's to the second
/// `Vec`. Assumes the inputs are already reduced mod 2<sup>`pow`</sup>.
///
/// Neither input slice may have trailing zeros.
///
/// Time: worst case O(`pow`)
///
/// Additional memory: worst case O(`pow`)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_two_sub::*;
///
/// let mut xs = vec![1, 2, 3];
/// let mut ys = vec![6, 7];
/// assert_eq!(limbs_mod_power_of_two_sub_in_place_either(&mut xs, &mut ys, 100), false);
/// assert_eq!(xs, &[4294967291, 4294967290, 2]);
/// assert_eq!(ys, &[6, 7]);
///
/// let mut xs = vec![6, 7];
/// let mut ys = vec![1, 2, 3];
/// assert_eq!(limbs_mod_power_of_two_sub_in_place_either(&mut xs, &mut ys, 100), true);
/// assert_eq!(xs, &[6, 7]);
/// assert_eq!(ys, &[5, 5, 4294967293, 15]);
/// ```
pub fn limbs_mod_power_of_two_sub_in_place_either(
    xs: &mut Vec<Limb>,
    ys: &mut Vec<Limb>,
    pow: u64,
) -> bool {
    if xs.len() >= ys.len() {
        limbs_mod_power_of_two_sub_in_place_left(xs, ys, pow);
        false
    } else {
        limbs_mod_power_of_two_sub_in_place_right(xs, ys, pow);
        true
    }
}

impl Natural {
    fn mod_power_of_two_sub_limb_ref(&self, y: Limb, pow: u64) -> Natural {
        match (&*self, y, pow) {
            (x, 0, _) => x.clone(),
            (&natural_zero!(), _, _) => Natural(Small(y)).mod_power_of_two_neg(pow),
            (&Natural(Small(small)), other, pow) if pow <= Limb::WIDTH => {
                Natural(Small(small.mod_power_of_two_sub(other, pow)))
            }
            (&Natural(Small(small)), other, _) => {
                let (diff, overflow) = small.overflowing_sub(other);
                if overflow {
                    let mut out = limbs_low_mask(pow);
                    out[0] = diff;
                    Natural(Large(out))
                } else {
                    Natural(Small(diff))
                }
            }
            (&Natural(Large(ref limbs)), other, _) => {
                Natural::from_owned_limbs_asc(limbs_sub_limb(limbs, other).0)
            }
        }
    }

    // other - self
    fn mod_power_of_two_right_sub_limb_ref(&self, y: Limb, pow: u64) -> Natural {
        match (&*self, y, pow) {
            (_, 0, _) => self.mod_power_of_two_neg(pow),
            (&natural_zero!(), _, _) => Natural(Small(y)),
            (&Natural(Small(small)), other, pow) if pow <= Limb::WIDTH => {
                Natural(Small(other.mod_power_of_two_sub(small, pow)))
            }
            (&Natural(Small(small)), other, _) => {
                let (diff, overflow) = other.overflowing_sub(small);
                if overflow {
                    let mut out = limbs_low_mask(pow);
                    out[0] = diff;
                    Natural(Large(out))
                } else {
                    Natural(Small(diff))
                }
            }
            (&Natural(Large(ref limbs)), other, _) => Natural::from_owned_limbs_asc(
                limbs_mod_power_of_two_limb_sub_limbs(other, limbs, pow),
            ),
        }
    }

    fn mod_power_of_two_sub_assign_limb(&mut self, y: Limb, pow: u64) {
        match (&mut *self, y, pow) {
            (_, 0, _) => {}
            (&mut natural_zero!(), _, _) => *self = Natural(Small(y)).mod_power_of_two_neg(pow),
            (&mut Natural(Small(ref mut small)), other, pow) if pow <= Limb::WIDTH => {
                small.mod_power_of_two_sub_assign(other, pow)
            }
            (&mut Natural(Small(ref mut small)), other, _) => {
                let (diff, overflow) = small.overflowing_sub(other);
                if overflow {
                    let mut out = limbs_low_mask(pow);
                    out[0] = diff;
                    *self = Natural(Large(out));
                } else {
                    *small = diff;
                }
            }
            (&mut Natural(Large(ref mut limbs)), other, _) => {
                limbs_sub_limb_in_place(limbs, other);
                self.trim();
            }
        }
    }

    // other -= self
    fn mod_power_of_two_right_sub_assign_limb(&mut self, other: Limb, pow: u64) {
        match (&mut *self, other, pow) {
            (_, 0, _) => self.mod_power_of_two_neg_assign(pow),
            (&mut natural_zero!(), _, _) => *self = Natural(Small(other)),
            (&mut Natural(Small(ref mut small)), other, pow) if pow <= Limb::WIDTH => {
                *small = other.mod_power_of_two_sub(*small, pow);
            }
            (&mut Natural(Small(ref mut small)), other, _) => {
                let (diff, overflow) = other.overflowing_sub(*small);
                if overflow {
                    let mut out = limbs_low_mask(pow);
                    out[0] = diff;
                    *self = Natural(Large(out))
                } else {
                    *small = diff
                }
            }
            (&mut Natural(Large(ref mut limbs)), other, _) => {
                limbs_mod_power_of_two_limb_sub_limbs_in_place(other, limbs, pow);
                self.trim();
            }
        }
    }
}

impl ModPowerOfTwoSub<Natural> for Natural {
    type Output = Natural;

    /// Subtracts a `Natural` from a `Natural` mod 2<sup>`pow`</sup>, taking both `Natural`s by
    /// value. Assumes the inputs are already reduced mod 2<sup>`pow`</sup>.
    ///
    /// Time: worst case O(`pow`)
    ///
    /// Additional memory: worst case O(`pow`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoSub;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(10u32).mod_power_of_two_sub(Natural::TWO, 4).to_string(), "8");
    /// assert_eq!(
    ///     Natural::from(56u32).mod_power_of_two_sub(Natural::from(123u32), 9).to_string(),
    ///     "445"
    /// );
    /// ```
    fn mod_power_of_two_sub(mut self, other: Natural, pow: u64) -> Natural {
        self.mod_power_of_two_sub_assign(other, pow);
        self
    }
}

impl<'a> ModPowerOfTwoSub<&'a Natural> for Natural {
    type Output = Natural;

    /// Subtracts a `Natural` from a `Natural` mod 2<sup>`pow`</sup>, taking the left `Natural` by
    /// value and the right `Natural` by reference. Assumes the inputs are already reduced mod
    /// 2<sup>`pow`</sup>.
    ///
    /// Time: worst case O(`pow`)
    ///
    /// Additional memory: worst case O(`pow`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoSub;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(10u32).mod_power_of_two_sub(&Natural::TWO, 4).to_string(), "8");
    /// assert_eq!(
    ///     Natural::from(56u32).mod_power_of_two_sub(&Natural::from(123u32), 9).to_string(),
    ///     "445"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_two_sub(mut self, other: &'a Natural, pow: u64) -> Natural {
        self.mod_power_of_two_sub_assign(other, pow);
        self
    }
}

impl<'a> ModPowerOfTwoSub<Natural> for &'a Natural {
    type Output = Natural;

    /// Subtracts a `Natural` from a `Natural` mod 2<sup>`pow`</sup>, taking the left `Natural` by
    /// reference and the right `Natural` by value. Assumes the inputs are already reduced mod
    /// 2<sup>`pow`</sup>.
    ///
    /// Time: worst case O(`pow`)
    ///
    /// Additional memory: worst case O(`pow`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoSub;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(10u32)).mod_power_of_two_sub(Natural::TWO, 4).to_string(), "8");
    /// assert_eq!(
    ///     (&Natural::from(56u32)).mod_power_of_two_sub(Natural::from(123u32), 9).to_string(),
    ///     "445"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_two_sub(self, mut other: Natural, pow: u64) -> Natural {
        match (self, &mut other) {
            (x, Natural(Small(y))) => x.mod_power_of_two_sub_limb_ref(*y, pow),
            (&Natural(Small(x)), y) => {
                y.mod_power_of_two_right_sub_assign_limb(x, pow);
                other
            }
            (&Natural(Large(ref xs)), &mut Natural(Large(ref mut ys))) => {
                limbs_mod_power_of_two_sub_in_place_right(xs, ys, pow);
                other.trim();
                other
            }
        }
    }
}

impl<'a, 'b> ModPowerOfTwoSub<&'a Natural> for &'b Natural {
    type Output = Natural;

    /// Subtracts a `Natural` from a `Natural` mod 2<sup>`pow`</sup>, taking both `Natural`s by
    /// reference. Assumes the inputs are already reduced mod 2<sup>`pow`</sup>.
    ///
    /// Time: worst case O(`pow`)
    ///
    /// Additional memory: worst case O(`pow`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoSub;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(10u32)).mod_power_of_two_sub(&Natural::TWO, 4).to_string(), "8");
    /// assert_eq!(
    ///     (&Natural::from(56u32)).mod_power_of_two_sub(&Natural::from(123u32), 9).to_string(),
    ///     "445"
    /// );
    /// ```
    fn mod_power_of_two_sub(self, other: &'a Natural, pow: u64) -> Natural {
        match (self, other) {
            (x, y) if x as *const Natural == y as *const Natural => natural_zero!(),
            (x, &Natural(Small(y))) => x.mod_power_of_two_sub_limb_ref(y, pow),
            (&Natural(Small(x)), y) => y.mod_power_of_two_right_sub_limb_ref(x, pow),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                Natural::from_owned_limbs_asc(limbs_mod_power_of_two_sub(xs, ys, pow))
            }
        }
    }
}

impl ModPowerOfTwoSubAssign<Natural> for Natural {
    /// Subtracts a `Natural` from a `Natural` mod 2<sup>`pow`</sup> in place, taking the `Natural`
    /// on the RHS by value. Assumes the inputs are already reduced mod 2<sup>`pow`</sup>.
    ///
    /// Time: worst case O(`pow`)
    ///
    /// Additional memory: worst case O(`pow`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoSubAssign;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_power_of_two_sub_assign(Natural::TWO, 4);
    /// assert_eq!(x.to_string(), "8");
    ///
    /// let mut x = Natural::from(56u32);
    /// x.mod_power_of_two_sub_assign(Natural::from(123u32), 9);
    /// assert_eq!(x.to_string(), "445");
    /// ```
    fn mod_power_of_two_sub_assign(&mut self, mut other: Natural, pow: u64) {
        match (&mut *self, &mut other) {
            (x, &mut Natural(Small(y))) => x.mod_power_of_two_sub_assign_limb(y, pow),
            (&mut Natural(Small(x)), y) => {
                y.mod_power_of_two_right_sub_assign_limb(x, pow);
                *self = other;
            }
            (&mut Natural(Large(ref mut xs)), Natural(Large(ref mut ys))) => {
                if limbs_mod_power_of_two_sub_in_place_either(xs, ys, pow) {
                    swap(xs, ys)
                }
                self.trim();
            }
        }
    }
}

impl<'a> ModPowerOfTwoSubAssign<&'a Natural> for Natural {
    /// Subtracts a `Natural` from a `Natural` mod 2<sup>`pow`</sup> in place, taking the `Natural`
    /// on the RHS by reference. Assumes the inputs are already reduced mod 2<sup>`pow`</sup>.
    ///
    /// Time: worst case O(`pow`)
    ///
    /// Additional memory: worst case O(`pow`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoSubAssign;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_power_of_two_sub_assign(&Natural::TWO, 4);
    /// assert_eq!(x.to_string(), "8");
    ///
    /// let mut x = Natural::from(56u32);
    /// x.mod_power_of_two_sub_assign(&Natural::from(123u32), 9);
    /// assert_eq!(x.to_string(), "445");
    /// ```
    fn mod_power_of_two_sub_assign(&mut self, other: &'a Natural, pow: u64) {
        match (&mut *self, other) {
            (x, y) if x as *const Natural == y as *const Natural => *self = natural_zero!(),
            (x, &Natural(Small(y))) => x.mod_power_of_two_sub_assign_limb(y, pow),
            (&mut Natural(Small(x)), y) => *self = y.mod_power_of_two_right_sub_limb_ref(x, pow),
            (&mut Natural(Large(ref mut xs)), &Natural(Large(ref ys))) => {
                limbs_mod_power_of_two_sub_in_place_left(xs, ys, pow);
                self.trim();
            }
        }
    }
}

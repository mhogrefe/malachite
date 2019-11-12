use malachite_base::comparison::Max;
use malachite_base::num::arithmetic::traits::{
    CeilingDivAssignNegMod, CeilingDivNegMod, DivAssignMod, DivAssignRem, DivMod, DivRem,
    WrappingAddAssign, WrappingSubAssign,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::Zero;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};

use natural::arithmetic::add_limb::limbs_slice_add_limb_in_place;
use natural::arithmetic::shl_u::{limbs_shl_to_out, limbs_slice_shl_in_place};
use natural::Natural::{self, Large, Small};
use platform::{DoubleLimb, Limb};

/// The highest bit of the input must be set.
///
/// Time: O(1)
///
/// Additional memory: O(1)
///
/// # Panics
/// Panics if `divisor` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_mod_limb::limbs_invert_limb;
///
/// assert_eq!(limbs_invert_limb(0x8000_0002), 0xffff_fff8);
/// assert_eq!(limbs_invert_limb(0xffff_fffe), 2);
/// ```
///
/// This is mpn_invert_limb from gmp-impl.h.
#[inline]
pub fn limbs_invert_limb(divisor: Limb) -> Limb {
    (DoubleLimb::join_halves(!divisor, Limb::MAX) / DoubleLimb::from(divisor)).lower_half()
}

/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// This is udiv_qrnnd_preinv from gmp-impl.h.
pub(crate) fn div_mod_by_preinversion(
    n_high: Limb,
    n_low: Limb,
    divisor: Limb,
    divisor_inverse: Limb,
) -> (Limb, Limb) {
    let (mut quotient_high, quotient_low) = (DoubleLimb::from(n_high)
        * DoubleLimb::from(divisor_inverse))
    .wrapping_add(DoubleLimb::join_halves(n_high.wrapping_add(1), n_low))
    .split_in_half();
    let mut remainder = n_low.wrapping_sub(quotient_high.wrapping_mul(divisor));
    if remainder > quotient_low {
        let (r_plus_d, overflow) = remainder.overflowing_add(divisor);
        if overflow {
            quotient_high.wrapping_sub_assign(1);
            remainder = r_plus_d;
        }
    } else if remainder >= divisor {
        quotient_high.wrapping_add_assign(1);
        remainder -= divisor;
    }
    (quotient_high, remainder)
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// quotient limbs and remainder of the `Natural` divided by a `Limb`. The divisor limb cannot be
/// zero and the limb slice must have at least two elements.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if the length of `limbs` is less than 2 or if `divisor` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_mod_limb::limbs_div_limb_mod;
///
/// assert_eq!(limbs_div_limb_mod(&[123, 456], 789), (vec![2_482_262_467, 0], 636));
/// assert_eq!(limbs_div_limb_mod(&[0xffff_ffff, 0xffff_ffff], 3),
///     (vec![0x5555_5555, 0x5555_5555], 0));
/// ```
///
/// This is mpn_divrem_1 from mpn/generic/divrem_1.c where qxn is 0, un > 1, and both results are
/// returned. Experiments show that DIVREM_1_NORM_THRESHOLD and DIVREM_1_UNNORM_THRESHOLD are
/// unnecessary (they would always be 0).
pub fn limbs_div_limb_mod(limbs: &[Limb], divisor: Limb) -> (Vec<Limb>, Limb) {
    let mut quotient_limbs = vec![0; limbs.len()];
    let remainder = limbs_div_limb_to_out_mod(&mut quotient_limbs, limbs, divisor);
    (quotient_limbs, remainder)
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the quotient of the `Natural` and a `Limb` to an output slice, and returns the
/// remainder. The output slice must be at least as long as the input slice. The divisor limb cannot
/// be zero and the input limb slice must have at least two elements.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `out` is shorter than `in_limbs`, the length of `in_limbs` is less than 2, or if
/// `divisor` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_mod_limb::limbs_div_limb_to_out_mod;
///
/// let mut out = vec![10, 10, 10, 10];
/// assert_eq!(limbs_div_limb_to_out_mod(&mut out, &[123, 456], 789), 636);
/// assert_eq!(out, &[2_482_262_467, 0, 10, 10]);
///
/// let mut out = vec![10, 10, 10, 10];
/// assert_eq!(limbs_div_limb_to_out_mod(&mut out, &[0xffff_ffff, 0xffff_ffff], 3), 0);
/// assert_eq!(out, &[0x5555_5555, 0x5555_5555, 10, 10]);
/// ```
///
/// This is mpn_divrem_1 from mpn/generic/divrem_1.c where qxn is 0 and un > 1. Experiments show
/// that DIVREM_1_NORM_THRESHOLD and DIVREM_1_UNNORM_THRESHOLD are unnecessary (they would always be
/// 0).
pub fn limbs_div_limb_to_out_mod(out: &mut [Limb], in_limbs: &[Limb], divisor: Limb) -> Limb {
    assert_ne!(divisor, 0);
    let len = in_limbs.len();
    assert!(len > 1);
    let out = &mut out[..len];
    let bits = divisor.leading_zeros();
    if bits == 0 {
        // High quotient limb is 0 or 1, skip a divide step.
        let (remainder, in_limbs_init) = in_limbs.split_last().unwrap();
        let mut remainder = *remainder;
        let (out_last, out_init) = out.split_last_mut().unwrap();
        *out_last = if remainder >= divisor {
            remainder -= divisor;
            1
        } else {
            0
        };
        // Multiply-by-inverse, divisor already normalized.
        let inverse = limbs_invert_limb(divisor);
        for (out_limb, &limb) in out_init.iter_mut().zip(in_limbs_init.iter()).rev() {
            let (quotient, new_remainder) =
                div_mod_by_preinversion(remainder, limb, divisor, inverse);
            *out_limb = quotient;
            remainder = new_remainder;
        }
        remainder
    } else {
        // Skip a division if high < divisor (high quotient 0). Testing here before normalizing will
        // still skip as often as possible.
        let (in_limbs_last, in_limbs_init) = in_limbs.split_last().unwrap();
        let (in_limbs, mut remainder) = if *in_limbs_last < divisor {
            *out.last_mut().unwrap() = 0;
            (in_limbs_init, *in_limbs_last)
        } else {
            (in_limbs, 0)
        };
        let divisor = divisor << bits;
        remainder <<= bits;
        let inverse = limbs_invert_limb(divisor);
        let (previous_limb, in_limbs_init) = in_limbs.split_last().unwrap();
        let mut previous_limb = *previous_limb;
        let cobits = Limb::WIDTH - bits;
        remainder |= previous_limb >> cobits;
        let (out_first, out_tail) = out.split_first_mut().unwrap();
        for (out_limb, &limb) in out_tail.iter_mut().zip(in_limbs_init.iter()).rev() {
            let nshift = (previous_limb << bits) | (limb >> cobits);
            let (quotient, new_remainder) =
                div_mod_by_preinversion(remainder, nshift, divisor, inverse);
            *out_limb = quotient;
            remainder = new_remainder;
            previous_limb = limb;
        }
        let (quotient, remainder) =
            div_mod_by_preinversion(remainder, previous_limb << bits, divisor, inverse);
        *out_first = quotient;
        remainder >> bits
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the quotient of the `Natural` and a `Limb` to the input slice and returns the
/// remainder. The divisor limb cannot be zero and the input limb slice must have at least two
/// elements.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if the length of `limbs` is less than 2 or if `divisor` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_mod_limb::limbs_div_limb_in_place_mod;
///
/// let mut limbs = vec![123, 456];
/// assert_eq!(limbs_div_limb_in_place_mod(&mut limbs, 789), 636);
/// assert_eq!(limbs, &[2_482_262_467, 0]);
///
/// let mut limbs = vec![0xffff_ffff, 0xffff_ffff];
/// assert_eq!(limbs_div_limb_in_place_mod(&mut limbs, 3), 0);
/// assert_eq!(limbs, &[0x5555_5555, 0x5555_5555]);
/// ```
///
/// This is mpn_divrem_1 from mpn/generic/divrem_1.c where qp == up, qxn is 0, and un > 1.
/// Experiments show that DIVREM_1_NORM_THRESHOLD and DIVREM_1_UNNORM_THRESHOLD are unnecessary
/// (they would always be 0).
pub fn limbs_div_limb_in_place_mod(limbs: &mut [Limb], divisor: Limb) -> Limb {
    assert_ne!(divisor, 0);
    let len = limbs.len();
    assert!(len > 1);
    let bits = divisor.leading_zeros();
    let (limbs_last, limbs_init) = limbs.split_last_mut().unwrap();
    if bits == 0 {
        // High quotient limb is 0 or 1, skip a divide step.
        let mut remainder = *limbs_last;
        *limbs_last = if remainder >= divisor {
            remainder -= divisor;
            1
        } else {
            0
        };
        // Multiply-by-inverse, divisor already normalized.
        let inverse = limbs_invert_limb(divisor);
        for limb in limbs_init.iter_mut().rev() {
            let (quotient, new_remainder) =
                div_mod_by_preinversion(remainder, *limb, divisor, inverse);
            *limb = quotient;
            remainder = new_remainder;
        }
        remainder
    } else {
        // Skip a division if high < divisor (high quotient 0). Testing here before normalizing will
        // still skip as often as possible.
        let (limbs, mut remainder) = if *limbs_last < divisor {
            let remainder = *limbs_last;
            *limbs_last = 0;
            (limbs_init, remainder)
        } else {
            (limbs, 0)
        };
        let divisor = divisor << bits;
        remainder <<= bits;
        let inverse = limbs_invert_limb(divisor);
        let last_index = limbs.len() - 1;
        let mut previous_limb = limbs[last_index];
        let cobits = Limb::WIDTH - bits;
        remainder |= previous_limb >> cobits;
        for i in (0..last_index).rev() {
            let limb = limbs[i];
            let shifted_limb = (previous_limb << bits) | (limb >> cobits);
            let (quotient, new_remainder) =
                div_mod_by_preinversion(remainder, shifted_limb, divisor, inverse);
            limbs[i + 1] = quotient;
            remainder = new_remainder;
            previous_limb = limb;
        }
        let (quotient, remainder) =
            div_mod_by_preinversion(remainder, previous_limb << bits, divisor, inverse);
        limbs[0] = quotient;
        remainder >> bits
    }
}

impl DivMod<Limb> for Natural {
    type DivOutput = Natural;
    type ModOutput = Limb;

    /// Divides a `Natural` by a `Limb`, taking the `Natural` by value and returning the quotient
    /// and remainder. The quotient is rounded towards negative infinity. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= r < `other`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(format!("{:?}", Natural::from(23u32).div_mod(10)), "(2, 3)");
    /// }
    /// ```
    #[inline]
    fn div_mod(mut self, other: Limb) -> (Natural, Limb) {
        let remainder = self.div_assign_rem(other);
        (self, remainder)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl DivMod<u32> for Natural {
    type DivOutput = Natural;
    type ModOutput = u32;

    #[inline]
    fn div_mod(self, other: u32) -> (Natural, u32) {
        let (quotient, remainder) = self.div_mod(Limb::from(other));
        (quotient, u32::wrapping_from(remainder))
    }
}

impl<'a> DivMod<Limb> for &'a Natural {
    type DivOutput = Natural;
    type ModOutput = Limb;

    /// Divides a `Natural` by a `Limb`, taking the `Natural` by reference and returning the
    /// quotient and remainder. The quotient is rounded towards negative infinity. The quotient and
    /// remainder satisfy `self` = q * `other` + r and 0 <= r < `other`. 0 <= r < `other`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(format!("{:?}", (&Natural::from(23u32)).div_mod(10)), "(2, 3)");
    /// }
    /// ```
    fn div_mod(self, other: Limb) -> (Natural, Limb) {
        if other == 0 {
            panic!("division by zero");
        } else if other == 1 {
            (self.clone(), 0)
        } else {
            match *self {
                Small(small) => {
                    let (quotient, remainder) = small.div_rem(other);
                    (Small(quotient), remainder)
                }
                Large(ref limbs) => {
                    let (quotient_limbs, remainder) = limbs_div_limb_mod(limbs, other);
                    let mut quotient = Large(quotient_limbs);
                    quotient.trim();
                    (quotient, remainder)
                }
            }
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> DivMod<u32> for &'a Natural {
    type DivOutput = Natural;
    type ModOutput = u32;

    #[inline]
    fn div_mod(self, other: u32) -> (Natural, u32) {
        let (quotient, remainder) = self.div_mod(Limb::from(other));
        (quotient, u32::wrapping_from(remainder))
    }
}

impl DivAssignMod<Limb> for Natural {
    type ModOutput = Limb;

    /// Divides a `Natural` by a `Limb` in place, returning the remainder. The quotient is rounded
    /// towards negative infinity. The quotient and remainder satisfy `self` = q * `other` + r and
    /// 0 <= r < `other`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivAssignMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Natural::from(23u32);
    ///     assert_eq!(x.div_assign_mod(10), 3);
    ///     assert_eq!(x.to_string(), "2");
    /// }
    /// ```
    fn div_assign_mod(&mut self, other: Limb) -> Limb {
        if other == 0 {
            panic!("division by zero");
        } else if other == 1 {
            0
        } else {
            let remainder = match *self {
                Small(ref mut small) => {
                    return small.div_assign_rem(other);
                }
                Large(ref mut limbs) => limbs_div_limb_in_place_mod(limbs, other),
            };
            self.trim();
            remainder
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl DivAssignMod<u32> for Natural {
    type ModOutput = u32;

    fn div_assign_mod(&mut self, other: u32) -> u32 {
        u32::wrapping_from(self.div_assign_mod(Limb::from(other)))
    }
}

impl DivMod<Natural> for Limb {
    type DivOutput = Limb;
    type ModOutput = Limb;

    /// Divides a `Limb` by a `Natural`, taking the `Natural` by value and returning the quotient
    /// and remainder. The quotient is rounded towards negative infinity. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= r < `other`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(23.div_mod(Natural::from(10u32)), (2, 3));
    /// }
    /// ```
    #[inline]
    fn div_mod(self, other: Natural) -> (Limb, Limb) {
        self.div_mod(&other)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl DivMod<Natural> for u32 {
    type DivOutput = u32;
    type ModOutput = u32;

    #[inline]
    fn div_mod(self, other: Natural) -> (u32, u32) {
        let (quotient, remainder) = Limb::from(self).div_mod(other);
        (u32::wrapping_from(quotient), u32::wrapping_from(remainder))
    }
}

impl<'a> DivMod<&'a Natural> for Limb {
    type DivOutput = Limb;
    type ModOutput = Limb;

    /// Divides a `Limb` by a `Natural`, taking the `Natural` by reference and returning the
    /// quotient and remainder. The quotient is rounded towards negative infinity. The quotient and
    /// remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(23.div_mod(&Natural::from(10u32)), (2, 3));
    /// }
    /// ```
    fn div_mod(self, other: &'a Natural) -> (Limb, Limb) {
        if *other == 0 as Limb {
            panic!("division by zero");
        } else if *other == 1 as Limb {
            (self, 0)
        } else {
            match *other {
                Small(small) => self.div_rem(small),
                Large(_) => (0, self),
            }
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> DivMod<&'a Natural> for u32 {
    type DivOutput = u32;
    type ModOutput = u32;

    #[inline]
    fn div_mod(self, other: &'a Natural) -> (u32, u32) {
        let (quotient, remainder) = Limb::from(self).div_mod(other);
        (u32::wrapping_from(quotient), u32::wrapping_from(remainder))
    }
}

impl DivAssignMod<Natural> for Limb {
    type ModOutput = Limb;

    /// Divides a `Limb` by a `Natural` in place, taking the `Natural` by value and returning the
    /// remainder. The quotient is rounded towards negative infinity. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= r < `other`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivAssignMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut n = 23;
    ///     assert_eq!(n.div_assign_mod(Natural::from(10u32)), 3);
    ///     assert_eq!(n, 2);
    /// }
    /// ```
    #[inline]
    fn div_assign_mod(&mut self, other: Natural) -> Limb {
        self.div_assign_mod(&other)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl DivAssignMod<Natural> for u32 {
    type ModOutput = u32;

    #[inline]
    fn div_assign_mod(&mut self, other: Natural) -> u32 {
        let (quotient, remainder) = self.div_mod(other);
        *self = quotient;
        remainder
    }
}

impl<'a> DivAssignMod<&'a Natural> for Limb {
    type ModOutput = Limb;

    /// Divides a `Limb` by a `Natural` in place, taking the `Natural` by reference and returning
    /// the remainder. The quotient is rounded towards negative infinity. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= r < `other`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivAssignMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut n = 23;
    ///     assert_eq!(n.div_assign_mod(&Natural::from(10u32)), 3);
    ///     assert_eq!(n, 2);
    /// }
    /// ```
    #[inline]
    fn div_assign_mod(&mut self, other: &'a Natural) -> Limb {
        let (quotient, remainder) = self.div_mod(other);
        *self = quotient;
        remainder
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> DivAssignMod<&'a Natural> for u32 {
    type ModOutput = u32;

    #[inline]
    fn div_assign_mod(&mut self, other: &'a Natural) -> u32 {
        let (quotient, remainder) = self.div_mod(other);
        *self = quotient;
        remainder
    }
}

impl DivRem<Limb> for Natural {
    type DivOutput = Natural;
    type RemOutput = Limb;

    /// Divides a `Natural` by a `Limb`, taking the `Natural` by value and returning the quotient
    /// and remainder. The quotient is rounded towards zero. The quotient and remainder satisfy
    /// `self` = q * `other` + r and 0 <= r < `other`. For `Natural`s, rem is equivalent to mod.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(format!("{:?}", Natural::from(23u32).div_rem(10)), "(2, 3)");
    /// }
    /// ```
    #[inline]
    fn div_rem(self, other: Limb) -> (Natural, Limb) {
        self.div_mod(other)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl DivRem<u32> for Natural {
    type DivOutput = Natural;
    type RemOutput = u32;

    #[inline]
    fn div_rem(self, other: u32) -> (Natural, u32) {
        self.div_mod(other)
    }
}

impl<'a> DivRem<Limb> for &'a Natural {
    type DivOutput = Natural;
    type RemOutput = Limb;

    /// Divides a `Natural` by a `Limb`, taking the `Natural` by reference and returning the
    /// quotient and remainder. The quotient is rounded towards zero. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= r < `other`. For `Natural`s, rem is equivalent to
    /// mod.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(format!("{:?}", (&Natural::from(23u32)).div_rem(10)), "(2, 3)");
    /// }
    /// ```
    #[inline]
    fn div_rem(self, other: Limb) -> (Natural, Limb) {
        self.div_mod(other)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> DivRem<u32> for &'a Natural {
    type DivOutput = Natural;
    type RemOutput = u32;

    #[inline]
    fn div_rem(self, other: u32) -> (Natural, u32) {
        self.div_mod(other)
    }
}

impl DivAssignRem<Limb> for Natural {
    type RemOutput = Limb;

    /// Divides a `Natural` by a `Limb` in place, returning the remainder. The quotient is rounded
    /// towards zero. The quotient and remainder satisfy `self` = q * `other` + r and
    /// 0 <= r < `other`. For `Natural`s, rem is equivalent to mod.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivAssignRem;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Natural::from(23u32);
    ///     assert_eq!(x.div_assign_rem(10), 3);
    ///     assert_eq!(x.to_string(), "2");
    /// }
    /// ```
    #[inline]
    fn div_assign_rem(&mut self, other: Limb) -> Limb {
        self.div_assign_mod(other)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl DivAssignRem<u32> for Natural {
    type RemOutput = u32;

    #[inline]
    fn div_assign_rem(&mut self, other: u32) -> u32 {
        self.div_assign_mod(other)
    }
}

impl DivRem<Natural> for Limb {
    type DivOutput = Limb;
    type RemOutput = Limb;

    /// Divides a `Limb` by a `Natural`, taking the `Natural` by value and returning the quotient
    /// and remainder. The quotient is rounded towards zero. The quotient and remainder satisfy
    /// `self` = q * `other` + r and 0 <= r < `other`. For `Natural`s, rem is equivalent to mod.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(23.div_rem(Natural::from(10u32)), (2, 3));
    /// }
    /// ```
    #[inline]
    fn div_rem(self, other: Natural) -> (Limb, Limb) {
        self.div_mod(other)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl DivRem<Natural> for u32 {
    type DivOutput = u32;
    type RemOutput = u32;

    #[inline]
    fn div_rem(self, other: Natural) -> (u32, u32) {
        self.div_mod(other)
    }
}

impl<'a> DivRem<&'a Natural> for Limb {
    type DivOutput = Limb;
    type RemOutput = Limb;

    /// Divides a `Limb` by a `Natural`, taking the `Natural` by reference and returning the
    /// quotient and remainder. The quotient is rounded towards zero. The quotient and remainder
    /// satisfy `self` = q * `other` + r and 0 <= r < `other`. For `Natural`s, rem is equivalent to
    /// mod.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(23.div_rem(&Natural::from(10u32)), (2, 3));
    /// }
    /// ```
    #[inline]
    fn div_rem(self, other: &'a Natural) -> (Limb, Limb) {
        self.div_mod(other)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> DivRem<&'a Natural> for u32 {
    type DivOutput = u32;
    type RemOutput = u32;

    #[inline]
    fn div_rem(self, other: &'a Natural) -> (u32, u32) {
        self.div_mod(other)
    }
}

impl DivAssignRem<Natural> for Limb {
    type RemOutput = Limb;

    /// Divides a `Limb` by a `Natural` in place, taking the `Natural` by value and returning the
    /// remainder. The quotient is rounded towards zero. The quotient and remainder satisfy
    /// `self` = q * `other` + r and 0 <= r < `other`. For `Natural`s, rem is equivalent to mod.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivAssignRem;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut n = 23;
    ///     assert_eq!(n.div_assign_rem(Natural::from(10u32)), 3);
    ///     assert_eq!(n, 2);
    /// }
    /// ```
    #[inline]
    fn div_assign_rem(&mut self, other: Natural) -> Limb {
        self.div_assign_mod(other)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl DivAssignRem<Natural> for u32 {
    type RemOutput = u32;

    #[inline]
    fn div_assign_rem(&mut self, other: Natural) -> u32 {
        self.div_assign_mod(other)
    }
}

impl<'a> DivAssignRem<&'a Natural> for Limb {
    type RemOutput = Limb;

    /// Divides a `Limb` by a `Natural` in place, taking the `Natural` by reference and returning
    /// the remainder. The quotient is rounded towards zero. The quotient and remainder satisfy
    /// `self` = q * `other` + r and 0 <= r < `other`. For `Natural`s, rem is equivalent to mod.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivAssignRem;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut n = 23;
    ///     assert_eq!(n.div_assign_rem(&Natural::from(10u32)), 3);
    ///     assert_eq!(n, 2);
    /// }
    /// ```
    #[inline]
    fn div_assign_rem(&mut self, other: &'a Natural) -> Limb {
        self.div_assign_mod(other)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> DivAssignRem<&'a Natural> for u32 {
    type RemOutput = u32;

    #[inline]
    fn div_assign_rem(&mut self, other: &'a Natural) -> u32 {
        self.div_assign_mod(other)
    }
}

impl CeilingDivNegMod<Limb> for Natural {
    type DivOutput = Natural;
    type ModOutput = Limb;

    /// Divides the `Natural` by a `Limb`, taking the `Natural` by value and returning the ceiling
    /// of the quotient and the remainder of the negative of the `Natural` divided by the `Limb`.
    /// The quotient and remainder satisfy `self` = q * `other` - r and 0 <= r < `other`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingDivNegMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 3 * 10 - 7 = 23
    ///     assert_eq!(format!("{:?}", Natural::from(23u32).ceiling_div_neg_mod(10)), "(3, 7)");
    /// }
    /// ```
    #[inline]
    fn ceiling_div_neg_mod(mut self, other: Limb) -> (Natural, Limb) {
        let remainder = self.ceiling_div_assign_neg_mod(other);
        (self, remainder)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl CeilingDivNegMod<u32> for Natural {
    type DivOutput = Natural;
    type ModOutput = u32;

    #[inline]
    fn ceiling_div_neg_mod(self, other: u32) -> (Natural, u32) {
        let (quotient, remainder) = self.ceiling_div_neg_mod(Limb::from(other));
        (quotient, u32::wrapping_from(remainder))
    }
}

impl<'a> CeilingDivNegMod<Limb> for &'a Natural {
    type DivOutput = Natural;
    type ModOutput = Limb;

    /// Divides a `Natural` by a `Limb`, taking the `Natural` by reference and returning the ceiling
    /// of the quotient and the remainder of the negative of the `Natural` divided by the `Limb`.
    /// The quotient and remainder satisfy `self` = q * `other` - r and 0 <= r < `other`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingDivNegMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 3 * 10 - 7 = 23
    ///     assert_eq!(format!("{:?}", (&Natural::from(23u32)).ceiling_div_neg_mod(10)), "(3, 7)");
    /// }
    /// ```
    fn ceiling_div_neg_mod(self, other: Limb) -> (Natural, Limb) {
        let (quotient, remainder) = self.div_mod(other);
        if remainder == 0 {
            (quotient, 0)
        } else {
            (quotient + 1 as Limb, other - remainder)
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> CeilingDivNegMod<u32> for &'a Natural {
    type DivOutput = Natural;
    type ModOutput = u32;

    #[inline]
    fn ceiling_div_neg_mod(self, other: u32) -> (Natural, u32) {
        let (quotient, remainder) = self.ceiling_div_neg_mod(Limb::from(other));
        (quotient, u32::wrapping_from(remainder))
    }
}

impl CeilingDivAssignNegMod<Limb> for Natural {
    type ModOutput = Limb;

    /// Divides a `Natural` by a `Limb` in place, taking the ceiling of the quotient and returning
    /// the remainder of the negative of the `Natural` divided by the `Limb`. The quotient and
    /// remainder satisfy `self` = q * `other` - r and 0 <= r < `other`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingDivAssignNegMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 3 * 10 - 7 = 23
    ///     let mut x = Natural::from(23u32);
    ///     assert_eq!(x.ceiling_div_assign_neg_mod(10), 7);
    ///     assert_eq!(x.to_string(), "3");
    /// }
    /// ```
    fn ceiling_div_assign_neg_mod(&mut self, other: Limb) -> Limb {
        let remainder = self.div_assign_mod(other);
        if remainder == 0 {
            0
        } else {
            *self += 1 as Limb;
            other - remainder
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl CeilingDivAssignNegMod<u32> for Natural {
    type ModOutput = u32;

    fn ceiling_div_assign_neg_mod(&mut self, other: u32) -> u32 {
        u32::wrapping_from(self.ceiling_div_assign_neg_mod(Limb::wrapping_from(other)))
    }
}

impl CeilingDivNegMod<Natural> for Limb {
    type DivOutput = Limb;
    type ModOutput = Natural;

    /// Divides a `Limb` by a `Natural`, taking the `Natural` by value and returning the ceiling of
    /// the quotient and the remainder of the negative of the `Limb` divided by the `Natural`. The
    /// quotient and remainder satisfy `self` = q * `other` - r and 0 <= r < `other`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingDivNegMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 3 * 10 - 7 = 23
    ///     assert_eq!(format!("{:?}", 23.ceiling_div_neg_mod(Natural::from(10u32))), "(3, 7)");
    /// }
    /// ```
    #[inline]
    fn ceiling_div_neg_mod(self, other: Natural) -> (Limb, Natural) {
        self.ceiling_div_neg_mod(&other)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl CeilingDivNegMod<Natural> for u32 {
    type DivOutput = u32;
    type ModOutput = Natural;

    #[inline]
    fn ceiling_div_neg_mod(self, other: Natural) -> (u32, Natural) {
        let (quotient, remainder) = Limb::wrapping_from(self).ceiling_div_neg_mod(other);
        (u32::wrapping_from(quotient), remainder)
    }
}

impl<'a> CeilingDivNegMod<&'a Natural> for Limb {
    type DivOutput = Limb;
    type ModOutput = Natural;

    /// Divides a `Limb` by a `Natural`, taking the `Natural` by reference and returning the ceiling
    /// of the quotient and the remainder of the negative of the `Limb` divided by the `Natural`.
    /// The quotient and remainder satisfy `self` = q * `other` - r and 0 <= r < `other`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingDivNegMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 3 * 10 - 7 = 23
    ///     assert_eq!(format!("{:?}", 23.ceiling_div_neg_mod(&Natural::from(10u32))), "(3, 7)");
    /// }
    /// ```
    fn ceiling_div_neg_mod(self, other: &'a Natural) -> (Limb, Natural) {
        let (quotient, remainder) = self.div_mod(other);
        if remainder == 0 {
            (quotient, Natural::ZERO)
        } else {
            (quotient + 1, other - remainder)
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> CeilingDivNegMod<&'a Natural> for u32 {
    type DivOutput = u32;
    type ModOutput = Natural;

    #[inline]
    fn ceiling_div_neg_mod(self, other: &'a Natural) -> (u32, Natural) {
        let (quotient, remainder) = Limb::wrapping_from(self).ceiling_div_neg_mod(other);
        (u32::wrapping_from(quotient), remainder)
    }
}

impl CeilingDivAssignNegMod<Natural> for Limb {
    type ModOutput = Natural;

    /// Divides a `Limb` by a `Natural` in place, taking the `Natural` by value, taking the ceiling
    /// of the quotient and returning the remainder of the negative of the `Limb` divided by the
    /// `Natural`. The quotient and remainder satisfy `self` = q * `other` - r and 0 <= r < `other`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingDivAssignNegMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 3 * 10 - 7 = 23
    ///     let mut x = 23;
    ///     assert_eq!(x.ceiling_div_assign_neg_mod(Natural::from(10u32)), 7);
    ///     assert_eq!(x.to_string(), "3");
    /// }
    /// ```
    #[inline]
    fn ceiling_div_assign_neg_mod(&mut self, other: Natural) -> Natural {
        self.ceiling_div_assign_neg_mod(&other)
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl CeilingDivAssignNegMod<Natural> for u32 {
    type ModOutput = Natural;

    #[inline]
    fn ceiling_div_assign_neg_mod(&mut self, other: Natural) -> Natural {
        let (quotient, remainder) = Limb::from(*self).ceiling_div_neg_mod(other);
        *self = u32::wrapping_from(quotient);
        remainder
    }
}

impl<'a> CeilingDivAssignNegMod<&'a Natural> for Limb {
    type ModOutput = Natural;

    /// Divides a `Limb` by a `Natural` in place, taking the `Natural` by reference, taking the
    /// ceiling of the quotient and returning the remainder of the negative of the `Limb` divided by
    /// the `Natural`. The quotient and remainder satisfy `self` = q * `other` - r and
    /// 0 <= r < `other`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingDivAssignNegMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     // 3 * 10 - 7 = 23
    ///     let mut x = 23;
    ///     assert_eq!(x.ceiling_div_assign_neg_mod(&Natural::from(10u32)), 7);
    ///     assert_eq!(x.to_string(), "3");
    /// }
    /// ```
    fn ceiling_div_assign_neg_mod(&mut self, other: &Natural) -> Natural {
        let remainder = self.div_assign_mod(other);
        if remainder == 0 {
            Natural::ZERO
        } else {
            *self += 1;
            other - remainder
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> CeilingDivAssignNegMod<&'a Natural> for u32 {
    type ModOutput = Natural;

    #[inline]
    fn ceiling_div_assign_neg_mod(&mut self, other: &'a Natural) -> Natural {
        let (quotient, remainder) = Limb::from(*self).ceiling_div_neg_mod(other);
        *self = u32::wrapping_from(quotient);
        remainder
    }
}

pub fn _limbs_div_limb_to_out_mod_naive(
    out: &mut [Limb],
    in_limbs: &[Limb],
    divisor: Limb,
) -> Limb {
    assert!(out.len() >= in_limbs.len());
    let divisor = DoubleLimb::from(divisor);
    let mut upper = 0;
    for (out_limb, &in_limb) in out.iter_mut().zip(in_limbs.iter()).rev() {
        let (quotient, remainder) = DoubleLimb::join_halves(upper, in_limb).div_rem(divisor);
        *out_limb = quotient.lower_half();
        upper = remainder.lower_half();
    }
    upper
}

pub fn _limbs_div_limb_in_place_mod_naive(limbs: &mut [Limb], divisor: Limb) -> Limb {
    let divisor = DoubleLimb::from(divisor);
    let mut upper = 0;
    for limb in limbs.iter_mut().rev() {
        let (quotient, remainder) = DoubleLimb::join_halves(upper, *limb).div_rem(divisor);
        *limb = quotient.lower_half();
        upper = remainder.lower_half();
    }
    upper
}

/// The high bit of `divisor` must be set.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// This is mpn_div_qr_1n_pi1 from mpn/generic/div_qr_1n_pi1.c with DIV_QR_1N_METHOD == 2, where
/// qp == up.
fn limbs_div_limb_normalized_in_place_mod(
    limbs: &mut [Limb],
    highest_limb: Limb,
    divisor: Limb,
    divisor_inverse: Limb,
) -> Limb {
    let len = limbs.len();
    if len == 1 {
        let (quotient, remainder) =
            div_mod_by_preinversion(highest_limb, limbs[0], divisor, divisor_inverse);
        limbs[0] = quotient;
        return remainder;
    }
    let power_of_two = divisor.wrapping_neg().wrapping_mul(divisor_inverse);
    let (mut quotient_high, mut quotient_low) =
        (DoubleLimb::from(divisor_inverse) * DoubleLimb::from(highest_limb)).split_in_half();
    quotient_high.wrapping_add_assign(highest_limb);
    let second_highest_limb = limbs[len - 1];
    limbs[len - 1] = quotient_high;
    let (sum, mut big_carry) = DoubleLimb::join_halves(second_highest_limb, limbs[len - 2])
        .overflowing_add(DoubleLimb::from(power_of_two) * DoubleLimb::from(highest_limb));
    let (mut sum_high, mut sum_low) = sum.split_in_half();
    for j in (0..len - 2).rev() {
        let (temp, remainder) =
            (DoubleLimb::from(sum_high) * DoubleLimb::from(divisor_inverse)).split_in_half();
        let mut quotient =
            DoubleLimb::from(sum_high) + DoubleLimb::from(temp) + DoubleLimb::from(quotient_low);
        quotient_low = remainder;
        if big_carry {
            quotient.wrapping_add_assign(DoubleLimb::join_halves(1, divisor_inverse));
            let (sum, carry) = sum_low.overflowing_add(power_of_two);
            sum_low = sum;
            if carry {
                sum_low.wrapping_sub_assign(divisor);
                quotient.wrapping_add_assign(1);
            }
        }
        let (quotient_higher, quotient_high) = quotient.split_in_half();
        limbs[j + 1] = quotient_high;
        assert!(!limbs_slice_add_limb_in_place(
            &mut limbs[j + 2..],
            quotient_higher,
        ));
        let (sum, carry) = DoubleLimb::join_halves(sum_low, limbs[j])
            .overflowing_add(DoubleLimb::from(sum_high) * DoubleLimb::from(power_of_two));
        sum_high = sum.upper_half();
        sum_low = sum.lower_half();
        big_carry = carry;
    }
    let mut quotient_high = 0;
    if big_carry {
        quotient_high += 1;
        sum_high.wrapping_sub_assign(divisor);
    }
    if sum_high >= divisor {
        quotient_high += 1;
        sum_high.wrapping_sub_assign(divisor);
    }
    let (temp, remainder) = div_mod_by_preinversion(sum_high, sum_low, divisor, divisor_inverse);
    let (quotient_high, quotient_low) = DoubleLimb::join_halves(quotient_high, quotient_low)
        .wrapping_add(DoubleLimb::from(temp))
        .split_in_half();
    assert!(!limbs_slice_add_limb_in_place(
        &mut limbs[1..],
        quotient_high,
    ));
    limbs[0] = quotient_low;
    remainder
}

/// The high bit of `divisor` must be set.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// This is mpn_div_qr_1n_pi1 from mpn/generic/div_qr_1n_pi1.c with DIV_QR_1N_METHOD == 2.
fn limbs_div_limb_normalized_to_out_mod(
    out: &mut [Limb],
    in_limbs: &[Limb],
    highest_limb: Limb,
    divisor: Limb,
    divisor_inverse: Limb,
) -> Limb {
    let len = in_limbs.len();
    if len == 1 {
        let (quotient, remainder) =
            div_mod_by_preinversion(highest_limb, in_limbs[0], divisor, divisor_inverse);
        out[0] = quotient;
        return remainder;
    }
    let power_of_two = divisor.wrapping_neg().wrapping_mul(divisor_inverse);
    let (mut quotient_high, mut quotient_low) =
        (DoubleLimb::from(divisor_inverse) * DoubleLimb::from(highest_limb)).split_in_half();
    quotient_high.wrapping_add_assign(highest_limb);
    out[len - 1] = quotient_high;
    let (sum, mut big_carry) = DoubleLimb::join_halves(in_limbs[len - 1], in_limbs[len - 2])
        .overflowing_add(DoubleLimb::from(power_of_two) * DoubleLimb::from(highest_limb));
    let (mut sum_high, mut sum_low) = sum.split_in_half();
    for j in (0..len - 2).rev() {
        let (temp, remainder) =
            (DoubleLimb::from(sum_high) * DoubleLimb::from(divisor_inverse)).split_in_half();
        let mut quotient =
            DoubleLimb::from(sum_high) + DoubleLimb::from(temp) + DoubleLimb::from(quotient_low);
        quotient_low = remainder;
        if big_carry {
            quotient.wrapping_add_assign(DoubleLimb::join_halves(1, divisor_inverse));
            let (sum, carry) = sum_low.overflowing_add(power_of_two);
            sum_low = sum;
            if carry {
                sum_low.wrapping_sub_assign(divisor);
                quotient.wrapping_add_assign(1);
            }
        }
        let (quotient_higher, quotient_high) = quotient.split_in_half();
        out[j + 1] = quotient_high;
        assert!(!limbs_slice_add_limb_in_place(
            &mut out[j + 2..],
            quotient_higher,
        ));
        let (sum, carry) = DoubleLimb::join_halves(sum_low, in_limbs[j])
            .overflowing_add(DoubleLimb::from(sum_high) * DoubleLimb::from(power_of_two));
        sum_high = sum.upper_half();
        sum_low = sum.lower_half();
        big_carry = carry;
    }
    let mut quotient_high = 0;
    if big_carry {
        quotient_high += 1;
        sum_high.wrapping_sub_assign(divisor);
    }
    if sum_high >= divisor {
        quotient_high += 1;
        sum_high.wrapping_sub_assign(divisor);
    }
    let (temp, remainder) = div_mod_by_preinversion(sum_high, sum_low, divisor, divisor_inverse);
    let (quotient_high, quotient_low) = DoubleLimb::join_halves(quotient_high, quotient_low)
        .wrapping_add(DoubleLimb::from(temp))
        .split_in_half();
    assert!(!limbs_slice_add_limb_in_place(&mut out[1..], quotient_high));
    out[0] = quotient_low;
    remainder
}

/// This is mpn_div_qr_1 from mpn/generic/div_qr_1.c where len > 1. Experiments show that this is
/// always slower than `_limbs_div_limb_to_out_mod`.
pub fn _limbs_div_limb_to_out_mod_alt(out: &mut [Limb], in_limbs: &[Limb], divisor: Limb) -> Limb {
    assert_ne!(divisor, 0);
    let len = in_limbs.len();
    assert!(len > 1);
    let out = &mut out[..len];
    assert!(out.len() >= len);
    let (highest_limb, in_limbs_init) = in_limbs.split_last().unwrap();
    let mut highest_limb = *highest_limb;
    let bits = divisor.leading_zeros();
    if bits == 0 {
        let (out_last, out_init) = out.split_last_mut().unwrap();
        *out_last = if highest_limb >= divisor {
            highest_limb -= divisor;
            1
        } else {
            0
        };
        let inverse = limbs_invert_limb(divisor);
        limbs_div_limb_normalized_to_out_mod(
            out_init,
            in_limbs_init,
            highest_limb,
            divisor,
            inverse,
        )
    } else {
        let divisor = divisor << bits;
        let highest_limb = limbs_shl_to_out(out, in_limbs, bits);
        let inverse = limbs_invert_limb(divisor);
        let (out_last, out_init) = out.split_last_mut().unwrap();
        let (quotient, remainder) =
            div_mod_by_preinversion(highest_limb, *out_last, divisor, inverse);
        *out_last = quotient;
        limbs_div_limb_normalized_in_place_mod(out_init, remainder, divisor, inverse) >> bits
    }
}

/// This is mpn_div_qr_1 from mpn/generic/div_qr_1.c where qp == up and len > 1. Experiments show
/// that this is always slower than `_limbs_div_limb_in_place_mod`.
pub fn _limbs_div_limb_in_place_mod_alt(limbs: &mut [Limb], divisor: Limb) -> Limb {
    assert_ne!(divisor, 0);
    let len = limbs.len();
    assert!(len > 1);
    let len_minus_1 = len - 1;
    let mut highest_limb = limbs[len_minus_1];
    let bits = divisor.leading_zeros();
    if bits == 0 {
        limbs[len_minus_1] = if highest_limb >= divisor {
            highest_limb -= divisor;
            1
        } else {
            0
        };
        let limb_inverse = limbs_invert_limb(divisor);
        limbs_div_limb_normalized_in_place_mod(
            &mut limbs[..len_minus_1],
            highest_limb,
            divisor,
            limb_inverse,
        )
    } else {
        let divisor = divisor << bits;
        let highest_limb = limbs_slice_shl_in_place(limbs, bits);
        let limb_inverse = limbs_invert_limb(divisor);
        let (quotient, remainder) =
            div_mod_by_preinversion(highest_limb, limbs[len_minus_1], divisor, limb_inverse);
        limbs[len_minus_1] = quotient;
        limbs_div_limb_normalized_in_place_mod(
            &mut limbs[..len_minus_1],
            remainder,
            divisor,
            limb_inverse,
        ) >> bits
    }
}

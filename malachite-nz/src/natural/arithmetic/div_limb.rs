use std::ops::{Div, DivAssign};

use malachite_base::num::arithmetic::traits::{DivRem, WrappingAddAssign, WrappingSubAssign};
use malachite_base::num::basic::integers::PrimitiveInteger;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};

use natural::arithmetic::add_limb::limbs_slice_add_limb_in_place;
use natural::arithmetic::div_mod_limb::{div_mod_by_preinversion, limbs_invert_limb};
use natural::arithmetic::shl_u::{limbs_shl_to_out, limbs_slice_shl_in_place};
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::{DoubleLimb, Limb};

/// Divide an number by a divisor of B - 1, where B is the limb base.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `out` is shorter than `xs`.
///
/// This is mpn_bdiv_dbm1c from mpn/generic/bdiv_dbm1c.c.
pub fn limbs_div_divisor_of_limb_max_with_carry_to_out(
    out: &mut [Limb],
    xs: &[Limb],
    divisor: Limb,
    mut carry: Limb,
) -> Limb {
    assert!(out.len() >= xs.len());
    let divisor = DoubleLimb::from(divisor);
    for (out_limb, &x) in out.iter_mut().zip(xs.iter()) {
        let (hi, lo) = (DoubleLimb::from(x) * divisor).split_in_half();
        let inner_carry = carry < lo;
        carry.wrapping_sub_assign(lo);
        *out_limb = carry;
        carry.wrapping_sub_assign(hi);
        if inner_carry {
            carry.wrapping_sub_assign(1);
        }
    }
    carry
}

/// Divide an number by a divisor of B - 1, where B is the limb base.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// This is mpn_bdiv_dbm1c from mpn/generic/bdiv_dbm1c.c, where qp == ap.
pub fn limbs_div_divisor_of_limb_max_with_carry_in_place(
    xs: &mut [Limb],
    divisor: Limb,
    mut carry: Limb,
) -> Limb {
    let divisor = DoubleLimb::from(divisor);
    for x in xs.iter_mut() {
        let (hi, lo) = (DoubleLimb::from(*x) * divisor).split_in_half();
        let inner_carry = carry < lo;
        carry.wrapping_sub_assign(lo);
        *x = carry;
        carry.wrapping_sub_assign(hi);
        if inner_carry {
            carry.wrapping_sub_assign(1);
        }
    }
    carry
}

/// Time: O(1)
///
/// Additional memory: O(1)
///
/// This is udiv_qrnnd_preinv from gmp-impl.h, but not computing the remainder.
fn div_by_preinversion(n_high: Limb, n_low: Limb, divisor: Limb, divisor_inverse: Limb) -> Limb {
    let (mut quotient_high, quotient_low) = (DoubleLimb::from(n_high)
        * DoubleLimb::from(divisor_inverse))
    .wrapping_add(DoubleLimb::join_halves(n_high.wrapping_add(1), n_low))
    .split_in_half();
    let mut remainder = n_low.wrapping_sub(quotient_high.wrapping_mul(divisor));
    if remainder > quotient_low {
        quotient_high.wrapping_sub_assign(1);
        remainder.wrapping_add_assign(divisor);
    }
    if remainder >= divisor {
        quotient_high.wrapping_add_assign(1);
    }
    quotient_high
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// quotient limbs of the `Natural` divided by a `Limb`. The divisor limb cannot be zero and the
/// limb slice must have at least two elements.
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
/// use malachite_nz::natural::arithmetic::div_limb::limbs_div_limb;
///
/// assert_eq!(limbs_div_limb(&[123, 456], 789), &[2_482_262_467, 0]);
/// assert_eq!(limbs_div_limb(&[0xffff_ffff, 0xffff_ffff], 3), &[0x5555_5555, 0x5555_5555]);
/// ```
///
/// This is mpn_div_qr_1 from mpn/generic/div_qr_1.c, where the quotient is returned, but not
/// computing the remainder.
pub fn limbs_div_limb(limbs: &[Limb], divisor: Limb) -> Vec<Limb> {
    let mut quotient_limbs = vec![0; limbs.len()];
    limbs_div_limb_to_out(&mut quotient_limbs, limbs, divisor);
    quotient_limbs
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the quotient of the `Natural` and a `Limb` to an output slice. The output slice must be
/// at least as long as the input slice. The divisor limb cannot be zero and the input limb slice
/// must have at least two elements.
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
/// use malachite_nz::natural::arithmetic::div_limb::limbs_div_limb_to_out;
///
/// let mut out = vec![10, 10, 10, 10];
/// limbs_div_limb_to_out(&mut out, &[123, 456], 789);
/// assert_eq!(out, &[2_482_262_467, 0, 10, 10]);
///
/// let mut out = vec![10, 10, 10, 10];
/// limbs_div_limb_to_out(&mut out, &[0xffff_ffff, 0xffff_ffff], 3);
/// assert_eq!(out, &[0x5555_5555, 0x5555_5555, 10, 10]);
/// ```
///
/// This is mpn_divrem_1 from mpn/generic/divrem_1.c, where qxn is 0 and un > 1, but not computing
/// the remainder.
pub fn limbs_div_limb_to_out(out: &mut [Limb], in_limbs: &[Limb], divisor: Limb) {
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
            let shifted_limb = (previous_limb << bits) | (limb >> cobits);
            let (quotient, new_remainder) =
                div_mod_by_preinversion(remainder, shifted_limb, divisor, inverse);
            *out_limb = quotient;
            remainder = new_remainder;
            previous_limb = limb;
        }
        *out_first = div_by_preinversion(remainder, previous_limb << bits, divisor, inverse);
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the quotient of the `Natural` and a `Limb` to the input slice. The divisor limb cannot
/// be zero and the input limb slice must have at least two elements.
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
/// use malachite_nz::natural::arithmetic::div_limb::limbs_div_limb_in_place;
///
/// let mut limbs = vec![123, 456];
/// limbs_div_limb_in_place(&mut limbs, 789);
/// assert_eq!(limbs, &[2_482_262_467, 0]);
///
/// let mut limbs = vec![0xffff_ffff, 0xffff_ffff];
/// limbs_div_limb_in_place(&mut limbs, 3);
/// assert_eq!(limbs, &[0x5555_5555, 0x5555_5555]);
/// ```
///
/// This is mpn_divrem_1 from mpn/generic/divrem_1.c, where qp == up, qxn is 0, and un > 1, but not
/// computing the remainder.
pub fn limbs_div_limb_in_place(limbs: &mut [Limb], divisor: Limb) {
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
        limbs[0] = div_by_preinversion(remainder, previous_limb << bits, divisor, inverse);
    }
}

impl Div<Limb> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Limb`, taking the `Natural` by value. The quotient is rounded
    /// towards negative infinity.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!((Natural::from(23u32) / 10).to_string(), "2");
    /// ```
    #[inline]
    fn div(mut self, other: Limb) -> Natural {
        self /= other;
        self
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl Div<u32> for Natural {
    type Output = Natural;

    #[inline]
    fn div(self, other: u32) -> Natural {
        self / Limb::from(other)
    }
}

impl<'a> Div<Limb> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Limb`, taking the `Natural` by reference. The quotient is rounded
    /// towards negative infinity.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!((&Natural::from(23u32) / 10).to_string(), "2");
    /// ```
    fn div(self, other: Limb) -> Natural {
        if other == 0 {
            panic!("division by zero");
        } else if other == 1 {
            self.clone()
        } else {
            match *self {
                Natural(Small(small)) => Natural(Small(small / other)),
                Natural(Large(ref limbs)) => {
                    let mut quotient = Natural(Large(limbs_div_limb(limbs, other)));
                    quotient.trim();
                    quotient
                }
            }
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> Div<u32> for &'a Natural {
    type Output = Natural;

    #[inline]
    fn div(self, other: u32) -> Natural {
        self / Limb::from(other)
    }
}

impl DivAssign<Limb> for Natural {
    /// Divides a `Natural` by a `Limb` in place. The quotient is rounded towards negative infinity.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Natural::from(23u32);
    /// x /= 10;
    /// assert_eq!(x.to_string(), "2");
    /// ```
    fn div_assign(&mut self, other: Limb) {
        if other == 0 {
            panic!("division by zero");
        } else if other != 1 {
            match *self {
                Natural(Small(ref mut small)) => {
                    *small /= other;
                    return;
                }
                Natural(Large(ref mut limbs)) => limbs_div_limb_in_place(limbs, other),
            }
            self.trim();
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl DivAssign<u32> for Natural {
    #[inline]
    fn div_assign(&mut self, other: u32) {
        *self /= Limb::from(other);
    }
}

impl Div<Natural> for Limb {
    type Output = Limb;

    /// Divides a `Limb` by a `Natural`, taking the `Natural` by value. The quotient is rounded
    /// towards negative infinity.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(23 / Natural::from(10u32), 2);
    /// ```
    fn div(self, other: Natural) -> Limb {
        if other == 0 as Limb {
            panic!("division by zero");
        } else {
            match other {
                Natural(Small(small)) => self / small,
                Natural(Large(_)) => 0,
            }
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl Div<Natural> for u32 {
    type Output = u32;

    #[inline]
    fn div(self, other: Natural) -> u32 {
        u32::wrapping_from(Limb::from(self) / other)
    }
}

impl<'a> Div<&'a Natural> for Limb {
    type Output = Limb;

    /// Divides a `u32` by a `Natural`, taking the `Natural` by reference. The quotient is rounded
    /// towards negative infinity.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(23 / &Natural::from(10u32), 2);
    /// ```
    fn div(self, other: &'a Natural) -> Limb {
        if *other == 0 as Limb {
            panic!("division by zero");
        } else {
            match *other {
                Natural(Small(small)) => self / small,
                Natural(Large(_)) => 0,
            }
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> Div<&'a Natural> for u32 {
    type Output = u32;

    #[inline]
    fn div(self, other: &'a Natural) -> u32 {
        u32::wrapping_from(Limb::from(self) / other)
    }
}

impl DivAssign<Natural> for Limb {
    /// Divides a `Limb` by a `Natural` in place, taking the `Natural` by value. The quotient is
    /// rounded towards negative infinity.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut n = 23;
    /// n /= Natural::from(10u32);
    /// assert_eq!(n, 2);
    /// ```
    #[inline]
    fn div_assign(&mut self, other: Natural) {
        *self /= &other;
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl DivAssign<Natural> for u32 {
    #[inline]
    fn div_assign(&mut self, other: Natural) {
        *self = u32::wrapping_from(Limb::from(*self) / other);
    }
}

impl<'a> DivAssign<&'a Natural> for Limb {
    /// Divides a `Limb` by a `Natural` in place, taking the `Natural` by reference. The quotient is
    /// rounded towards negative infinity.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut n = 23;
    /// n /= &Natural::from(10u32);
    /// assert_eq!(n, 2);
    /// ```
    #[inline]
    fn div_assign(&mut self, other: &'a Natural) {
        *self = *self / other;
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> DivAssign<&'a Natural> for u32 {
    #[inline]
    fn div_assign(&mut self, other: &'a Natural) {
        *self = u32::wrapping_from(Limb::from(*self) / other);
    }
}

/// Divides using the naive (schoolbook) algorithm.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
fn _limbs_div_in_place_naive(limbs: &mut [Limb], limb: Limb) {
    let limb = DoubleLimb::from(limb);
    let mut upper = 0;
    for x in limbs.iter_mut().rev() {
        let lower = *x;
        let (q, r) = DoubleLimb::join_halves(upper, lower).div_rem(limb);
        *x = q.lower_half();
        upper = r.lower_half();
    }
}

impl Natural {
    pub fn _div_limb_naive(mut self, other: Limb) -> Natural {
        self._div_assign_limb_naive(other);
        self
    }

    pub fn _div_assign_limb_naive(&mut self, other: Limb) {
        if other == 0 {
            panic!("division by zero");
        } else if other != 1 {
            match *self {
                Natural(Small(ref mut small)) => {
                    *small /= other;
                    return;
                }
                Natural(Large(ref mut limbs)) => _limbs_div_in_place_naive(limbs, other),
            }
            self.trim();
        }
    }
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
/// qp == up, but not computing the remainder.
fn limbs_div_limb_normalized_in_place(
    limbs: &mut [Limb],
    high_limb: Limb,
    divisor: Limb,
    divisor_inverse: Limb,
) {
    let len = limbs.len();
    if len == 1 {
        limbs[0] = div_by_preinversion(high_limb, limbs[0], divisor, divisor_inverse);
        return;
    }
    let power_of_two = divisor.wrapping_neg().wrapping_mul(divisor_inverse);
    let (mut quotient_high, mut quotient_low) =
        (DoubleLimb::from(divisor_inverse) * DoubleLimb::from(high_limb)).split_in_half();
    quotient_high.wrapping_add_assign(high_limb);
    let second_highest_limb = limbs[len - 1];
    limbs[len - 1] = quotient_high;
    let (sum, mut big_carry) = DoubleLimb::join_halves(second_highest_limb, limbs[len - 2])
        .overflowing_add(DoubleLimb::from(power_of_two) * DoubleLimb::from(high_limb));
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
    let temp = div_by_preinversion(sum_high, sum_low, divisor, divisor_inverse);
    let (quotient_high, quotient_low) = DoubleLimb::join_halves(quotient_high, quotient_low)
        .wrapping_add(DoubleLimb::from(temp))
        .split_in_half();
    assert!(!limbs_slice_add_limb_in_place(
        &mut limbs[1..],
        quotient_high,
    ));
    limbs[0] = quotient_low;
}

/// The high bit of `divisor` must be set.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// This is mpn_div_qr_1n_pi1 from mpn/generic/div_qr_1n_pi1.c with DIV_QR_1N_METHOD == 2, but not
/// computing the remainder.
fn limbs_div_limb_normalized_to_out(
    out: &mut [Limb],
    in_limbs: &[Limb],
    high_limb: Limb,
    divisor: Limb,
    divisor_inverse: Limb,
) {
    let len = in_limbs.len();
    if len == 1 {
        out[0] = div_by_preinversion(high_limb, in_limbs[0], divisor, divisor_inverse);
        return;
    }
    let power_of_two = divisor.wrapping_neg().wrapping_mul(divisor_inverse);
    let (mut quotient_high, mut quotient_low) =
        (DoubleLimb::from(divisor_inverse) * DoubleLimb::from(high_limb)).split_in_half();
    quotient_high.wrapping_add_assign(high_limb);
    out[len - 1] = quotient_high;
    let (sum, mut big_carry) = DoubleLimb::join_halves(in_limbs[len - 1], in_limbs[len - 2])
        .overflowing_add(DoubleLimb::from(power_of_two) * DoubleLimb::from(high_limb));
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
    let temp = div_by_preinversion(sum_high, sum_low, divisor, divisor_inverse);
    let (quotient_high, quotient_low) = DoubleLimb::join_halves(quotient_high, quotient_low)
        .wrapping_add(DoubleLimb::from(temp))
        .split_in_half();
    assert!(!limbs_slice_add_limb_in_place(&mut out[1..], quotient_high));
    out[0] = quotient_low;
}

/// This is mpn_div_qr_1 from mpn/generic/div_qr_1.c, but not computing the remainder. Experiments
/// show that this is always slower than `_limbs_div_limb_to_out`.
pub fn _limbs_div_limb_to_out_alt(out: &mut [Limb], in_limbs: &[Limb], divisor: Limb) {
    assert_ne!(divisor, 0);
    let len = in_limbs.len();
    assert!(len > 1);
    assert!(out.len() >= len);
    let len_minus_1 = len - 1;
    let mut highest_limb = in_limbs[len_minus_1];
    let bits = divisor.leading_zeros();
    if bits == 0 {
        out[len_minus_1] = if highest_limb >= divisor {
            highest_limb -= divisor;
            1
        } else {
            0
        };
        let limb_inverse = limbs_invert_limb(divisor);
        limbs_div_limb_normalized_to_out(
            out,
            &in_limbs[..len_minus_1],
            highest_limb,
            divisor,
            limb_inverse,
        )
    } else {
        let divisor = divisor << bits;
        let highest_limb = limbs_shl_to_out(out, in_limbs, bits);
        let limb_inverse = limbs_invert_limb(divisor);
        let (quotient, remainder) =
            div_mod_by_preinversion(highest_limb, out[len_minus_1], divisor, limb_inverse);
        out[len_minus_1] = quotient;
        limbs_div_limb_normalized_in_place(
            &mut out[..len_minus_1],
            remainder,
            divisor,
            limb_inverse,
        )
    }
}

/// This is mpn_div_qr_1 from mpn/generic/div_qr_1.c, where qp == up, but not computing the
/// remainder. Experiments show that this is always slower than `_limbs_div_limb_in_place`.
pub fn _limbs_div_limb_in_place_alt(limbs: &mut [Limb], divisor: Limb) {
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
        limbs_div_limb_normalized_in_place(
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
        limbs_div_limb_normalized_in_place(
            &mut limbs[..len_minus_1],
            remainder,
            divisor,
            limb_inverse,
        )
    }
}

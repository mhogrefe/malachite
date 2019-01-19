use malachite_base::misc::Max;
use malachite_base::num::{DivRem, JoinHalves, SplitInHalf, WrappingAddAssign, WrappingSubAssign};
use natural::arithmetic::add_limb::limbs_slice_add_limb_in_place;
use natural::arithmetic::div_mod_limb::div_mod_by_preinversion;
use natural::arithmetic::shl_u::{limbs_shl_to_out, limbs_slice_shl_in_place};
use natural::Natural::{self, Large, Small};
use platform::{DoubleLimb, Limb};
use std::ops::{Div, DivAssign};

// These functions are adapted from udiv_qrnnd_preinv, mpn_div_qr_1n_pi1, and mpn_div_qr_1 in GMP
// 6.1.2.

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

// high bit of divisor must be set
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
            quotient_higher
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
        quotient_high
    ));
    limbs[0] = quotient_low;
}

// high bit of divisor must be set
fn limbs_div_limb_normalized_to_out(
    out_limbs: &mut [Limb],
    in_limbs: &[Limb],
    high_limb: Limb,
    divisor: Limb,
    divisor_inverse: Limb,
) {
    let len = in_limbs.len();
    if len == 1 {
        out_limbs[0] = div_by_preinversion(high_limb, in_limbs[0], divisor, divisor_inverse);
        return;
    }
    let power_of_two = divisor.wrapping_neg().wrapping_mul(divisor_inverse);
    let (mut quotient_high, mut quotient_low) =
        (DoubleLimb::from(divisor_inverse) * DoubleLimb::from(high_limb)).split_in_half();
    quotient_high.wrapping_add_assign(high_limb);
    out_limbs[len - 1] = quotient_high;
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
        out_limbs[j + 1] = quotient_high;
        assert!(!limbs_slice_add_limb_in_place(
            &mut out_limbs[j + 2..],
            quotient_higher
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
    assert!(!limbs_slice_add_limb_in_place(
        &mut out_limbs[1..],
        quotient_high
    ));
    out_limbs[0] = quotient_low;
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
/// Panics if `out_limbs` is shorter than `in_limbs`, the length of `in_limbs` is less than 2, or if
/// `divisor` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_limb::limbs_div_limb_to_out;
///
/// let mut out_limbs = vec![10, 10, 10, 10];
/// limbs_div_limb_to_out(&mut out_limbs, &[123, 456], 789);
/// assert_eq!(out_limbs, &[2_482_262_467, 0, 10, 10]);
///
/// let mut out_limbs = vec![10, 10, 10, 10];
/// limbs_div_limb_to_out(&mut out_limbs, &[0xffff_ffff, 0xffff_ffff], 3);
/// assert_eq!(out_limbs, &[0x5555_5555, 0x5555_5555, 10, 10]);
/// ```
pub fn limbs_div_limb_to_out(out_limbs: &mut [Limb], in_limbs: &[Limb], mut divisor: Limb) {
    assert!(divisor > 0);
    let len = in_limbs.len();
    assert!(len > 1);
    assert!(out_limbs.len() >= len);
    let len_minus_1 = len - 1;
    let mut highest_limb = in_limbs[len_minus_1];
    let bits = divisor.leading_zeros();
    if bits == 0 {
        out_limbs[len_minus_1] = if highest_limb >= divisor {
            highest_limb -= divisor;
            1
        } else {
            0
        };
        let limb_inverse =
            (DoubleLimb::join_halves(!divisor, Limb::MAX) / DoubleLimb::from(divisor)).lower_half();
        limbs_div_limb_normalized_to_out(
            out_limbs,
            &in_limbs[..len_minus_1],
            highest_limb,
            divisor,
            limb_inverse,
        )
    } else {
        divisor <<= bits;
        let highest_limb = limbs_shl_to_out(out_limbs, in_limbs, bits);
        let limb_inverse =
            (DoubleLimb::join_halves(!divisor, Limb::MAX) / DoubleLimb::from(divisor)).lower_half();
        let (quotient, remainder) =
            div_mod_by_preinversion(highest_limb, out_limbs[len_minus_1], divisor, limb_inverse);
        out_limbs[len_minus_1] = quotient;
        limbs_div_limb_normalized_in_place(
            &mut out_limbs[..len_minus_1],
            remainder,
            divisor,
            limb_inverse,
        )
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
pub fn limbs_div_limb_in_place(limbs: &mut [Limb], mut divisor: Limb) {
    assert!(divisor > 0);
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
        let limb_inverse =
            (DoubleLimb::join_halves(!divisor, Limb::MAX) / DoubleLimb::from(divisor)).lower_half();
        limbs_div_limb_normalized_in_place(
            &mut limbs[..len_minus_1],
            highest_limb,
            divisor,
            limb_inverse,
        )
    } else {
        divisor <<= bits;
        let highest_limb = limbs_slice_shl_in_place(limbs, bits);
        let limb_inverse =
            (DoubleLimb::join_halves(!divisor, Limb::MAX) / DoubleLimb::from(divisor)).lower_half();
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
    fn div(mut self, other: Limb) -> Natural {
        self /= other;
        self
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
                Small(small) => Small(small / other),
                Large(ref limbs) => {
                    let mut quotient = Large(limbs_div_limb(limbs, other));
                    quotient.trim();
                    quotient
                }
            }
        }
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
                Small(ref mut small) => {
                    *small /= other;
                    return;
                }
                Large(ref mut limbs) => limbs_div_limb_in_place(limbs, other),
            }
            self.trim();
        }
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
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(23 / Natural::from(10u32), 2);
    /// ```
    fn div(self, other: Natural) -> Limb {
        if other == 0 {
            panic!("division by zero");
        } else {
            match other {
                Small(small) => self / small,
                Large(_) => 0,
            }
        }
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
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(23 / &Natural::from(10u32), 2);
    /// ```
    fn div(self, other: &'a Natural) -> Limb {
        if *other == 0 {
            panic!("division by zero");
        } else {
            match *other {
                Small(small) => self / small,
                Large(_) => 0,
            }
        }
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
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut n = 23;
    /// n /= Natural::from(10u32);
    /// assert_eq!(n, 2);
    /// ```
    fn div_assign(&mut self, other: Natural) {
        *self /= &other;
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
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut n = 23;
    /// n /= &Natural::from(10u32);
    /// assert_eq!(n, 2);
    /// ```
    fn div_assign(&mut self, other: &'a Natural) {
        *self = *self / other;
    }
}

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
                Small(ref mut small) => {
                    *small /= other;
                    return;
                }
                Large(ref mut limbs) => _limbs_div_in_place_naive(limbs, other),
            }
            self.trim();
        }
    }
}

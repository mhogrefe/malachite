use std::cmp::Ordering;
use std::ops::{Rem, RemAssign};

use malachite_base::comparison::Max;
use malachite_base::num::arithmetic::traits::{
    DivAssignMod, DivMod, Mod, ModAssign, NegMod, NegModAssign, WrappingAddAssign,
    WrappingSubAssign,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};

use natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
use natural::arithmetic::div_mod::{
    _limbs_div_mod_divide_and_conquer_helper, _limbs_div_mod_schoolbook,
    limbs_div_mod_by_two_limb_normalized, limbs_div_mod_three_limb_by_two_limb,
    limbs_two_limb_inverse_helper,
};
use natural::arithmetic::mul::{limbs_mul_greater_to_out, limbs_mul_to_out};
use natural::arithmetic::shl_u::limbs_shl_to_out;
use natural::arithmetic::sub::limbs_sub_same_length_in_place_left;
use natural::arithmetic::sub_limb::limbs_sub_limb_in_place;
use natural::arithmetic::sub_mul_limb::limbs_sub_mul_limb_same_length_in_place_left;
use natural::comparison::ord::limbs_cmp_same_length;
use natural::Natural;
use platform::{DoubleLimb, Limb, DC_DIV_QR_THRESHOLD};

/// Computes the remainder of `[n_2, n_1, n_0]` / `[d_1, d_0]`. Requires the highest bit of `d_1` to
/// be set, and `[n_2, n_1]` < `[d_1, d_0]`. `inverse` is the inverse of `[d_1, d_0]` computed by
/// `limbs_two_limb_inverse_helper`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::div_mod::limbs_two_limb_inverse_helper;
/// use malachite_nz::natural::arithmetic::mod_op::limbs_mod_three_limb_by_two_limb;
///
/// let d_1 = 0x8000_0004;
/// let d_0 = 5;
/// assert_eq!(
///     limbs_mod_three_limb_by_two_limb(
///         1, 2, 3, d_1, d_0,
///         limbs_two_limb_inverse_helper(d_1, d_0)),
///     0x7fff_fffd_ffff_fffe
/// );
///
/// let d_1 = 0x8000_0000;
/// let d_0 = 0;
/// assert_eq!(
///     limbs_mod_three_limb_by_two_limb(
///         2, 0x4000_0000, 4, d_1, d_0,
///         limbs_two_limb_inverse_helper(d_1, d_0)),
///     0x4000_0000_0000_0004
/// );
/// ```
///
/// This is udiv_qr_3by2 from gmp-impl.h, returning only the remainder.
pub fn limbs_mod_three_limb_by_two_limb(
    n_2: Limb,
    n_1: Limb,
    n_0: Limb,
    d_1: Limb,
    d_0: Limb,
    inverse: Limb,
) -> DoubleLimb {
    let (q, q_lo) = (DoubleLimb::from(n_2) * DoubleLimb::from(inverse))
        .wrapping_add(DoubleLimb::join_halves(n_2, n_1))
        .split_in_half();
    let d = DoubleLimb::join_halves(d_1, d_0);
    // Compute the two most significant limbs of n - q * d
    let r = DoubleLimb::join_halves(n_1.wrapping_sub(d_1.wrapping_mul(q)), n_0)
        .wrapping_sub(d)
        .wrapping_sub(DoubleLimb::from(d_0) * DoubleLimb::from(q));
    // Conditionally adjust the remainder
    if r.upper_half() >= q_lo {
        let (r_plus_d, overflow) = r.overflowing_add(d);
        if overflow {
            return r_plus_d;
        }
    } else if r >= d {
        return r.wrapping_sub(d);
    }
    r
}

/// Divides `ns` by `ds`, returning the limbs of the remainder. `ds` must have length 2, `ns` must
/// have length at least 2, and the most significant bit of `ds[1]` must be set.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `ns.len()`
///
/// # Panics
/// Panics if `ds` does not have length 2, `ns` has length less than 2, `qs` has length less than
/// `ns.len() - 2`, or `ds[1]` does not have its highest bit set.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mod_op::limbs_mod_by_two_limb_normalized;
///
/// assert_eq!(
///     limbs_mod_by_two_limb_normalized(&[1, 2, 3, 4, 5], &[3, 0x8000_0000]),
///     (166, 2147483626)
/// );
/// ```
///
/// This is mpn_divrem_2 from mpn/generic/divrem_2.c, returning the two limbs of the remainder.
pub fn limbs_mod_by_two_limb_normalized(ns: &[Limb], ds: &[Limb]) -> (Limb, Limb) {
    assert_eq!(ds.len(), 2);
    let n_len = ns.len();
    assert!(n_len >= 2);
    let n_limit = n_len - 2;
    assert!(ds[1].get_highest_bit());
    let d_1 = ds[1];
    let d_0 = ds[0];
    let d = DoubleLimb::join_halves(d_1, d_0);
    let mut r = DoubleLimb::join_halves(ns[n_limit + 1], ns[n_limit]);
    if r >= d {
        r.wrapping_sub_assign(d);
    }
    let (mut r_1, mut r_0) = r.split_in_half();
    let inverse = limbs_two_limb_inverse_helper(d_1, d_0);
    for &n in ns[..n_limit].iter().rev() {
        let (new_r_1, new_r_0) =
            limbs_mod_three_limb_by_two_limb(r_1, r_0, n, d_1, d_0, inverse).split_in_half();
        r_1 = new_r_1;
        r_0 = new_r_0;
    }
    (r_0, r_1)
}

/// Divides `ns` by `ds` and writes the `ds.len()` limbs of the remainder to `ns`. `ds` must have
/// length greater than 2, `ns` must be at least as long as `ds`, and the most significant bit of
/// `ds` must be set. `inverse` should be the result of `limbs_two_limb_inverse_helper` applied to
/// the two highest limbs of the denominator.
///
/// Time: worst case O(d * (n - d + 1)); also, O(n ^ 2)
///
/// Additional memory: worst case O(1)
///
/// where n = `ns.len()`
///       d = `ds.len()`
///
/// # Panics
/// Panics if `ds` has length smaller than 3, `ns` is shorter than `ds`, or the last limb of `ds`
/// does not have its highest bit set.
///
/// This is mpn_sbpi1_div_qr from mpn/generic/sbpi1_div_qr.c, where only the remainder is
/// calculated.
pub fn _limbs_mod_schoolbook(ns: &mut [Limb], ds: &[Limb], inverse: Limb) {
    let d_len = ds.len();
    assert!(d_len > 2);
    let n_len = ns.len();
    assert!(n_len >= d_len);
    let d_1 = ds[d_len - 1];
    assert!(d_1.get_highest_bit());
    let d_0 = ds[d_len - 2];
    let ds_except_last = &ds[..d_len - 1];
    let ds_except_last_two = &ds[..d_len - 2];
    {
        let ns_hi = &mut ns[n_len - d_len..];
        if limbs_cmp_same_length(ns_hi, ds) >= Ordering::Equal {
            limbs_sub_same_length_in_place_left(ns_hi, ds);
        }
    }
    let mut n_1 = ns[n_len - 1];
    for i in (d_len..n_len).rev() {
        let j = i - d_len;
        if n_1 == d_1 && ns[i - 1] == d_0 {
            limbs_sub_mul_limb_same_length_in_place_left(&mut ns[j..i], ds, Limb::MAX);
            n_1 = ns[i - 1]; // update n_1, last loop's value will now be invalid
        } else {
            let carry;
            {
                let (ns_lo, ns_hi) = ns.split_at_mut(i - 2);
                let (q, new_n) = limbs_div_mod_three_limb_by_two_limb(
                    n_1, ns_hi[1], ns_hi[0], d_1, d_0, inverse,
                );
                let (new_n_1, mut n_0) = new_n.split_in_half();
                n_1 = new_n_1;
                let local_carry_1 = limbs_sub_mul_limb_same_length_in_place_left(
                    &mut ns_lo[j..],
                    ds_except_last_two,
                    q,
                );
                let local_carry_2 = n_0 < local_carry_1;
                n_0.wrapping_sub_assign(local_carry_1);
                carry = local_carry_2 && n_1 == 0;
                if local_carry_2 {
                    n_1.wrapping_sub_assign(1);
                }
                ns_hi[0] = n_0;
            }
            if carry {
                n_1.wrapping_add_assign(d_1);
                if limbs_slice_add_same_length_in_place_left(&mut ns[j..i - 1], ds_except_last) {
                    n_1.wrapping_add_assign(1);
                }
            }
        }
    }
    ns[d_len - 1] = n_1;
}

/// Time: worst case O(n * log(n) ^ 2 * log(log(n)))
///
/// Additional memory: worst case O(n * log(n) ^ 2)
///
/// where n = `ds.len()`
///
/// This is mpn_dcpi1_div_qr_n from mpn/generic/dcpi1_div_qr.c, where only the remainder is
/// calculated.
fn _limbs_mod_divide_and_conquer_helper(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    inverse: Limb,
    scratch: &mut [Limb],
) {
    let n = ds.len();
    let lo = n >> 1; // floor(n / 2)
    let hi = n - lo; // ceil(n / 2)
    {
        let qs_hi = &mut qs[lo..];
        let (ds_lo, ds_hi) = ds.split_at(lo);
        let highest_q = if hi < DC_DIV_QR_THRESHOLD {
            _limbs_div_mod_schoolbook(qs_hi, &mut ns[2 * lo..2 * n], ds_hi, inverse)
        } else {
            _limbs_div_mod_divide_and_conquer_helper(
                qs_hi,
                &mut ns[2 * lo..],
                ds_hi,
                inverse,
                scratch,
            )
        };
        let qs_hi = &mut qs_hi[..hi];
        limbs_mul_greater_to_out(scratch, qs_hi, ds_lo);
        let ns_lo = &mut ns[..n + lo];
        let mut carry = if limbs_sub_same_length_in_place_left(&mut ns_lo[lo..], &scratch[..n]) {
            1
        } else {
            0
        };
        if highest_q && limbs_sub_same_length_in_place_left(&mut ns_lo[n..], ds_lo) {
            carry += 1;
        }
        while carry != 0 {
            limbs_sub_limb_in_place(qs_hi, 1);
            if limbs_slice_add_same_length_in_place_left(&mut ns_lo[lo..], ds) {
                carry -= 1;
            }
        }
    }
    let (ds_lo, ds_hi) = ds.split_at(hi);
    let q_lo = if lo < DC_DIV_QR_THRESHOLD {
        _limbs_div_mod_schoolbook(qs, &mut ns[hi..n + lo], ds_hi, inverse)
    } else {
        _limbs_div_mod_divide_and_conquer_helper(qs, &mut ns[hi..], ds_hi, inverse, scratch)
    };
    let qs_lo = &mut qs[..lo];
    let ns_lo = &mut ns[..n];
    limbs_mul_greater_to_out(scratch, ds_lo, qs_lo);
    let mut carry = if limbs_sub_same_length_in_place_left(ns_lo, &scratch[..n]) {
        1
    } else {
        0
    };
    if q_lo && limbs_sub_same_length_in_place_left(&mut ns_lo[lo..], ds_lo) {
        carry += 1;
    }
    while carry != 0 {
        if limbs_slice_add_same_length_in_place_left(ns_lo, ds) {
            carry -= 1;
        }
    }
}

/// Time: worst case O(n * log(n) ^ 2 * log(log(n)))
///
/// Additional memory: worst case O(n * log(n) ^ 2)
///
/// where n = `ds.len()`
///
/// # Panics
/// Panics if `ds` has length smaller than 6, `ns.len()` is less than `ds.len()` + 3, `qs` has
/// length less than `ns.len()` - `ds.len()`, or the last limb of `ds` does not have its highest bit
/// set.
///
/// This is mpn_dcpi1_div_qr from mpn/generic/dcpi1_div_qr.c, where only the remainder is
/// calculated.
pub fn _limbs_mod_divide_and_conquer(qs: &mut [Limb], ns: &mut [Limb], ds: &[Limb], inverse: Limb) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(d_len >= 6); // to adhere to _limbs_div_mod_schoolbook's limits
    assert!(n_len >= d_len + 3); // to adhere to _limbs_div_mod_schoolbook's limits
    let a = d_len - 1;
    let d_1 = ds[a];
    let b = d_len - 2;
    assert!(d_1.get_highest_bit());
    let mut scratch = vec![0; d_len];
    let q_len = n_len - d_len;
    if q_len > d_len {
        let q_len_mod_d_len = {
            let mut m = q_len % d_len;
            if m == 0 {
                m = d_len;
            }
            m
        };
        // Perform the typically smaller block first.
        {
            // point at low limb of next quotient block
            let qs = &mut qs[q_len - q_len_mod_d_len..q_len];
            if q_len_mod_d_len == 1 {
                // Handle highest_q up front, for simplicity.
                let ns = &mut ns[q_len - 1..];
                {
                    let ns = &mut ns[1..];
                    if limbs_cmp_same_length(ns, ds) >= Ordering::Equal {
                        assert!(!limbs_sub_same_length_in_place_left(ns, ds));
                    }
                }
                // A single iteration of schoolbook: One 3/2 division, followed by the bignum update
                // and adjustment.
                let (last_n, ns) = ns.split_last_mut().unwrap();
                let n_2 = *last_n;
                let mut n_1 = ns[a];
                let mut n_0 = ns[b];
                let d_0 = ds[b];
                assert!(n_2 < d_1 || n_2 == d_1 && n_1 <= d_0);
                let mut q;
                if n_2 == d_1 && n_1 == d_0 {
                    // TODO This branch is untested!
                    q = Limb::MAX;
                    assert_eq!(limbs_sub_mul_limb_same_length_in_place_left(ns, ds, q), n_2);
                } else {
                    let (new_q, new_n) =
                        limbs_div_mod_three_limb_by_two_limb(n_2, n_1, n_0, d_1, d_0, inverse);
                    q = new_q;
                    let (new_n_1, new_n_0) = new_n.split_in_half();
                    n_1 = new_n_1;
                    n_0 = new_n_0;
                    // d_len > 2 because of precondition. No need to check
                    let local_carry_1 =
                        limbs_sub_mul_limb_same_length_in_place_left(&mut ns[..b], &ds[..b], q);
                    let local_carry_2 = n_0 < local_carry_1;
                    n_0.wrapping_sub_assign(local_carry_1);
                    let carry = local_carry_2 && n_1 == 0;
                    if local_carry_2 {
                        n_1.wrapping_sub_assign(1);
                    }
                    ns[b] = n_0;
                    let (last_n, ns) = ns.split_last_mut().unwrap();
                    if carry {
                        n_1.wrapping_add_assign(d_1);
                        if limbs_slice_add_same_length_in_place_left(ns, &ds[..a]) {
                            n_1.wrapping_add_assign(1);
                        }
                        q.wrapping_sub_assign(1);
                    }
                    *last_n = n_1;
                }
                qs[0] = q;
            } else {
                // Do a 2 * q_len_mod_d_len / q_len_mod_d_len division
                let (ds_lo, ds_hi) = ds.split_at(d_len - q_len_mod_d_len);
                let highest_q = {
                    let ns = &mut ns[n_len - (q_len_mod_d_len << 1)..];
                    if q_len_mod_d_len == 2 {
                        limbs_div_mod_by_two_limb_normalized(qs, ns, ds_hi)
                    } else if q_len_mod_d_len < DC_DIV_QR_THRESHOLD {
                        _limbs_div_mod_schoolbook(qs, ns, ds_hi, inverse)
                    } else {
                        _limbs_div_mod_divide_and_conquer_helper(
                            qs,
                            ns,
                            ds_hi,
                            inverse,
                            &mut scratch,
                        )
                    }
                };
                if q_len_mod_d_len != d_len {
                    limbs_mul_to_out(&mut scratch, qs, ds_lo);
                    let ns = &mut ns[q_len - q_len_mod_d_len..n_len - q_len_mod_d_len];
                    let mut carry = if limbs_sub_same_length_in_place_left(ns, &scratch) {
                        1
                    } else {
                        0
                    };
                    if highest_q {
                        if limbs_sub_same_length_in_place_left(&mut ns[q_len_mod_d_len..], ds_lo) {
                            carry += 1;
                        }
                    }
                    while carry != 0 {
                        limbs_sub_limb_in_place(qs, 1);
                        if limbs_slice_add_same_length_in_place_left(ns, ds) {
                            carry -= 1;
                        }
                    }
                }
            }
        }
        // offset is a multiple of d_len
        let mut offset = n_len.checked_sub(d_len + q_len_mod_d_len).unwrap();
        while offset != 0 {
            offset -= d_len;
            _limbs_mod_divide_and_conquer_helper(
                &mut qs[offset..],
                &mut ns[offset..],
                ds,
                inverse,
                &mut scratch,
            );
        }
    } else {
        let m = d_len - q_len;
        let (ds_lo, ds_hi) = ds.split_at(m);
        let highest_q = if q_len < DC_DIV_QR_THRESHOLD {
            _limbs_div_mod_schoolbook(qs, &mut ns[m..], ds_hi, inverse)
        } else {
            _limbs_div_mod_divide_and_conquer_helper(qs, &mut ns[m..], ds_hi, inverse, &mut scratch)
        };
        if m != 0 {
            let qs = &mut qs[..q_len];
            let ns = &mut ns[..d_len];
            limbs_mul_to_out(&mut scratch, &qs, ds_lo);
            let mut carry = if limbs_sub_same_length_in_place_left(ns, &scratch) {
                1
            } else {
                0
            };
            if highest_q && limbs_sub_same_length_in_place_left(&mut ns[q_len..], ds_lo) {
                carry += 1;
            }
            while carry != 0 {
                if limbs_slice_add_same_length_in_place_left(ns, ds) {
                    carry -= 1;
                }
            }
        }
    }
}

//TODO not pub
pub fn _limbs_mod_by_two_limb(ns: &[Limb], ds: &[Limb]) -> (Limb, Limb) {
    let n_len = ns.len();
    let ds_1 = ds[1];
    let bits = ds_1.leading_zeros();
    if bits == 0 {
        limbs_mod_by_two_limb_normalized(ns, ds)
    } else {
        let ds_0 = ds[0];
        let cobits = Limb::WIDTH - bits;
        let mut ns_shifted = vec![0; n_len + 1];
        let ns_shifted = &mut ns_shifted;
        let carry = limbs_shl_to_out(ns_shifted, ns, bits);
        let ds_shifted = &mut [ds_0 << bits, (ds_1 << bits) | (ds_0 >> cobits)];
        let (r_0, r_1) = if carry == 0 {
            limbs_mod_by_two_limb_normalized(&ns_shifted[..n_len], ds_shifted)
        } else {
            ns_shifted[n_len] = carry;
            limbs_mod_by_two_limb_normalized(ns_shifted, ds_shifted)
        };
        ((r_0 >> bits) | (r_1 << cobits), r_1 >> bits)
    }
}

impl Mod<Natural> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking both `Natural`s by value and returning the
    /// remainder. The quotient and remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(Natural::from(23u32).mod_op(Natural::from(10u32)).to_string(), "3");
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     assert_eq!(
    ///          Natural::from_str("1000000000000000000000000").unwrap()
    ///              .mod_op(Natural::from_str("1234567890987").unwrap()).to_string(),
    ///          "530068894399"
    ///     );
    /// }
    /// ```
    #[inline]
    fn mod_op(self, other: Natural) -> Natural {
        self % other
    }
}

impl<'a> Mod<&'a Natural> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking the first `Natural` by value and the second by
    /// reference, and returning the remainder. The quotient and remainder satisfy `self` =
    /// q * `other` + r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!(Natural::from(23u32).mod_op(&Natural::from(10u32)).to_string(), "3");
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     assert_eq!(
    ///          Natural::from_str("1000000000000000000000000").unwrap()
    ///              .mod_op(&Natural::from_str("1234567890987").unwrap()).to_string(),
    ///          "530068894399"
    ///     );
    /// }
    /// ```
    #[inline]
    fn mod_op(self, other: &'a Natural) -> Natural {
        self % other
    }
}

impl<'a> Mod<Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking the first `Natural` by reference and the second
    /// by value, and returning the remainder. The quotient and remainder satisfy `self` =
    /// q * `other` + r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!((&Natural::from(23u32)).mod_op(Natural::from(10u32)).to_string(), "3");
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     assert_eq!(
    ///          (&Natural::from_str("1000000000000000000000000").unwrap())
    ///              .mod_op(Natural::from_str("1234567890987").unwrap()).to_string(),
    ///          "530068894399"
    ///     );
    /// }
    /// ```
    #[inline]
    fn mod_op(self, other: Natural) -> Natural {
        self % other
    }
}

impl<'a, 'b> Mod<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking both `Natural`s by reference and returning the
    /// remainder. The quotient and remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!((&Natural::from(23u32)).mod_op(&Natural::from(10u32)).to_string(), "3");
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     assert_eq!(
    ///          (&Natural::from_str("1000000000000000000000000").unwrap())
    ///              .mod_op(&Natural::from_str("1234567890987").unwrap()).to_string(),
    ///          "530068894399"
    ///     );
    /// }
    /// ```
    #[inline]
    fn mod_op(self, other: &'b Natural) -> Natural {
        self % other
    }
}

impl ModAssign<Natural> for Natural {
    /// Divides a `Natural` by a `Natural` in place, taking the second `Natural` by value and
    /// replacing `self` with the remainder. The quotient and remainder satisfy `self` =
    /// q * `other` + r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModAssign;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Natural::from(23u32);
    ///     x.mod_assign(Natural::from(10u32));
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    ///     x.mod_assign(Natural::from_str("1234567890987").unwrap());
    ///     assert_eq!(x.to_string(), "530068894399");
    /// }
    /// ```
    #[inline]
    fn mod_assign(&mut self, other: Natural) {
        *self %= other;
    }
}

impl<'a> ModAssign<&'a Natural> for Natural {
    /// Divides a `Natural` by a `Natural` in place, taking the second `Natural` by reference and
    /// replacing `self` with the remainder. The quotient and remainder satisfy `self` =
    /// q * `other` + r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModAssign;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Natural::from(23u32);
    ///     x.mod_assign(&Natural::from(10u32));
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    ///     x.mod_assign(&Natural::from_str("1234567890987").unwrap());
    ///     assert_eq!(x.to_string(), "530068894399");
    /// }
    /// ```
    fn mod_assign(&mut self, other: &'a Natural) {
        *self %= other;
    }
}

impl Rem<Natural> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking both `Natural`s by value and returning the
    /// remainder. The quotient and remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
    /// For `Natural`s, rem is equivalent to mod.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!((Natural::from(23u32) % Natural::from(10u32)).to_string(), "3");
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     assert_eq!(
    ///         (Natural::from_str("1000000000000000000000000").unwrap() %
    ///                 Natural::from_str("1234567890987").unwrap()).to_string(),
    ///         "530068894399"
    ///     );
    /// }
    /// ```
    #[inline]
    fn rem(mut self, other: Natural) -> Natural {
        self %= other;
        self
    }
}

impl<'a> Rem<&'a Natural> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking the first `Natural` by value and the second by
    /// reference, and returning the remainder. The quotient and remainder satisfy `self` =
    /// q * `other` + r and 0 <= r < `other`. For `Natural`s, rem is equivalent to mod.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!((Natural::from(23u32) % &Natural::from(10u32)).to_string(), "3");
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     assert_eq!(
    ///         (Natural::from_str("1000000000000000000000000").unwrap() %
    ///                 &Natural::from_str("1234567890987").unwrap()).to_string(),
    ///         "530068894399"
    ///     );
    /// }
    /// ```
    #[inline]
    fn rem(mut self, other: &'a Natural) -> Natural {
        self %= other;
        self
    }
}

impl<'a> Rem<Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking the first `Natural` by reference and the second
    /// by value, and returning the remainder. The quotient and remainder satisfy `self` =
    /// q * `other` + r and 0 <= r < `other`. For `Natural`s, rem is equivalent to mod.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!((&Natural::from(23u32) % Natural::from(10u32)).to_string(), "3");
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     assert_eq!(
    ///         (&Natural::from_str("1000000000000000000000000").unwrap() %
    ///                 Natural::from_str("1234567890987").unwrap()).to_string(),
    ///         "530068894399"
    ///     );
    /// }
    /// ```
    #[inline]
    fn rem(self, other: Natural) -> Natural {
        self.div_mod(other).1
    }
}

impl<'a, 'b> Rem<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking both `Natural`s by reference and returning the
    /// remainder. The quotient and remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
    /// For `Natural`s, rem is equivalent to mod.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     assert_eq!((&Natural::from(23u32) % &Natural::from(10u32)).to_string(), "3");
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     assert_eq!(
    ///         (&Natural::from_str("1000000000000000000000000").unwrap() %
    ///                 &Natural::from_str("1234567890987").unwrap()).to_string(),
    ///         "530068894399"
    ///     );
    /// }
    /// ```
    #[inline]
    fn rem(self, other: &'b Natural) -> Natural {
        self.div_mod(other).1
    }
}

impl RemAssign<Natural> for Natural {
    /// Divides a `Natural` by a `Natural` in place, taking the second `Natural` by value and
    /// replacing `self` with the remainder. The quotient and remainder satisfy `self` =
    /// q * `other` + r and 0 <= r < `other`. For `Natural`s, rem is equivalent to mod.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Natural::from(23u32);
    ///     x %= Natural::from(10u32);
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    ///     x %= Natural::from_str("1234567890987").unwrap();
    ///     assert_eq!(x.to_string(), "530068894399");
    /// }
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: Natural) {
        *self = self.div_assign_mod(other);
    }
}

impl<'a> RemAssign<&'a Natural> for Natural {
    /// Divides a `Natural` by a `Natural` in place, taking the second `Natural` by reference and
    /// replacing `self` with the remainder. The quotient and remainder satisfy `self` =
    /// q * `other` + r and 0 <= r < `other`. For `Natural`s, rem is equivalent to mod.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 2 * 10 + 3 = 23
    ///     let mut x = Natural::from(23u32);
    ///     x %= &Natural::from(10u32);
    ///     assert_eq!(x.to_string(), "3");
    ///
    ///     // 810000006723 * 1234567890987 + 530068894399 = 1000000000000000000000000
    ///     let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    ///     x %= &Natural::from_str("1234567890987").unwrap();
    ///     assert_eq!(x.to_string(), "530068894399");
    /// }
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: &'a Natural) {
        *self = self.div_assign_mod(other);
    }
}

impl NegMod<Natural> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking both `Natural`s by value and returning the
    /// remainder of the negative of the first `Natural` divided by the second. The quotient and
    /// remainder satisfy `self` = q * `other` - r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::NegMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 10 - 7 = 23
    ///     assert_eq!(Natural::from(23u32).neg_mod(Natural::from(10u32)).to_string(), "7");
    ///
    ///     // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    ///     assert_eq!(
    ///          Natural::from_str("1000000000000000000000000").unwrap()
    ///                 .neg_mod(Natural::from_str("1234567890987").unwrap()).to_string(),
    ///          "704498996588"
    ///     );
    /// }
    /// ```
    #[inline]
    fn neg_mod(mut self, other: Natural) -> Natural {
        self.neg_mod_assign(other);
        self
    }
}

impl<'a> NegMod<&'a Natural> for Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking the first `Natural` by value and the second by
    /// reference, and returning the remainder of the negative of the first `Natural` divided by the
    /// second. The quotient and remainder satisfy `self` = q * `other` - r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::NegMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 10 - 7 = 23
    ///     assert_eq!(Natural::from(23u32).neg_mod(&Natural::from(10u32)).to_string(), "7");
    ///
    ///     // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    ///     assert_eq!(
    ///          Natural::from_str("1000000000000000000000000").unwrap()
    ///                 .neg_mod(&Natural::from_str("1234567890987").unwrap()).to_string(),
    ///          "704498996588"
    ///     );
    /// }
    /// ```
    #[inline]
    fn neg_mod(mut self, other: &'a Natural) -> Natural {
        self.neg_mod_assign(other);
        self
    }
}

impl<'a> NegMod<Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking the first `Natural` by reference and the second
    /// by value, and returning the remainder of the negative of the first `Natural` divided by the
    /// second. The quotient and remainder satisfy `self` = q * `other` - r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::NegMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 10 - 7 = 23
    ///     assert_eq!((&Natural::from(23u32)).neg_mod(Natural::from(10u32)).to_string(), "7");
    ///
    ///     // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    ///     assert_eq!(
    ///          (&Natural::from_str("1000000000000000000000000").unwrap())
    ///                 .neg_mod(Natural::from_str("1234567890987").unwrap()).to_string(),
    ///          "704498996588"
    ///     );
    /// }
    /// ```
    #[inline]
    fn neg_mod(self, other: Natural) -> Natural {
        let remainder = self % &other;
        if remainder == 0 as Limb {
            remainder
        } else {
            other - remainder
        }
    }
}

impl<'a, 'b> NegMod<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a `Natural` by a `Natural`, taking both `Natural`s by reference and returning the
    /// remainder of the negative of the first `Natural` divided by the second. The quotient and
    /// remainder satisfy `self` = q * `other` - r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::NegMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 10 - 7 = 23
    ///     assert_eq!((&Natural::from(23u32)).neg_mod(&Natural::from(10u32)).to_string(), "7");
    ///
    ///     // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    ///     assert_eq!(
    ///          (&Natural::from_str("1000000000000000000000000").unwrap())
    ///                 .neg_mod(&Natural::from_str("1234567890987").unwrap()).to_string(),
    ///          "704498996588"
    ///     );
    /// }
    /// ```
    #[inline]
    fn neg_mod(self, other: &'b Natural) -> Natural {
        let remainder = self % other;
        if remainder == 0 as Limb {
            remainder
        } else {
            other - remainder
        }
    }
}

impl NegModAssign<Natural> for Natural {
    /// Divides a `Natural` by a `Natural` in place, taking the second `Natural` by value, replacing
    /// `self` with the remainder of the negative of the first `Natural` divided by the second. The
    /// quotient and remainder satisfy `self` = q * `other` - r and 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::NegModAssign;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 10 - 7 = 23
    ///     let mut x = Natural::from(23u32);
    ///     x.neg_mod_assign(Natural::from(10u32));
    ///     assert_eq!(x.to_string(), "7");
    ///
    ///     // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    ///     let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    ///     x.neg_mod_assign(Natural::from_str("1234567890987").unwrap());
    ///     assert_eq!(x.to_string(), "704498996588");
    /// }
    /// ```
    #[inline]
    fn neg_mod_assign(&mut self, other: Natural) {
        *self %= &other;
        if *self != 0 as Limb {
            self.sub_right_assign_no_panic(&other);
        }
    }
}

impl<'a> NegModAssign<&'a Natural> for Natural {
    /// Divides a `Natural` by a `Natural` in place, taking the second `Natural` by reference,
    /// and replacing `self` with the remainder of the negative of the first `Natural` divided by
    /// the second. The quotient and remainder satisfy `self` = q * `other` - r and
    /// 0 <= r < `other`.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::NegModAssign;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     // 3 * 10 - 7 = 23
    ///     let mut x = Natural::from(23u32);
    ///     x.neg_mod_assign(&Natural::from(10u32));
    ///     assert_eq!(x.to_string(), "7");
    ///
    ///     // 810000006724 * 1234567890987 - 704498996588 = 1000000000000000000000000
    ///     let mut x = Natural::from_str("1000000000000000000000000").unwrap();
    ///     x.neg_mod_assign(&Natural::from_str("1234567890987").unwrap());
    ///     assert_eq!(x.to_string(), "704498996588");
    /// }
    /// ```
    fn neg_mod_assign(&mut self, other: &'a Natural) {
        *self %= other;
        if *self != 0 as Limb {
            self.sub_right_assign_no_panic(other);
        }
    }
}

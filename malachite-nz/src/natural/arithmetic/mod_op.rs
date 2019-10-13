use std::ops::{Rem, RemAssign};

use malachite_base::num::arithmetic::traits::{
    DivAssignMod, DivMod, Mod, ModAssign, NegMod, NegModAssign, WrappingSubAssign,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};

use natural::arithmetic::div_mod::limbs_two_limb_inverse_helper;
use natural::arithmetic::shl_u::limbs_shl_to_out;
use natural::Natural;
use platform::{DoubleLimb, Limb};

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

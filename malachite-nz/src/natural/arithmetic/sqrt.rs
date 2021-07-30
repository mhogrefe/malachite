use malachite_base::num::arithmetic::sqrt::sqrt_rem_newton;
use malachite_base::num::arithmetic::traits::{
    CeilingSqrt, CeilingSqrtAssign, CheckedSqrt, FloorSqrt, FloorSqrtAssign, ModPowerOf2, Parity,
    PowerOf2, ShrRound, SqrtRem, SqrtRemAssign, Square, WrappingAddAssign, WrappingSubAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Iverson, One, Two};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::slices::slice_test_zero;
use natural::arithmetic::add::{
    limbs_slice_add_limb_in_place, limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use natural::arithmetic::div::{
    _limbs_div_barrett_approx, _limbs_div_barrett_approx_scratch_len,
    _limbs_div_divide_and_conquer_approx, _limbs_div_schoolbook_approx,
};
use natural::arithmetic::div_mod::{
    limbs_div_limb_to_out_mod, limbs_div_mod_qs_to_out_rs_to_ns, limbs_two_limb_inverse_helper,
};
use natural::arithmetic::mul::limbs_mul_greater_to_out;
use natural::arithmetic::shl::limbs_shl_to_out;
use natural::arithmetic::shr::{limbs_shr_to_out, limbs_slice_shr_in_place};
use natural::arithmetic::square::limbs_square_to_out;
use natural::arithmetic::sub::{limbs_sub_limb_in_place, limbs_sub_same_length_in_place_left};
use natural::comparison::ord::limbs_cmp_same_length;
use natural::Natural;
use platform::{Limb, SignedLimb, DC_DIVAPPR_Q_THRESHOLD, MU_DIVAPPR_Q_THRESHOLD};
use std::cmp::Ordering;

/// Returns (sqrt, r_hi, r_lo) such that [n_lo, n_hi] = sqrt ^ 2 + [r_lo, r_hi].
///
/// This is mpn_sqrtrem2 from mpn/generic/sqrtrem.c, GMP 6.2.1.
#[doc(hidden)]
pub fn _sqrt_rem_2_newton(n_hi: Limb, n_lo: Limb) -> (Limb, bool, Limb) {
    assert!(n_hi.leading_zeros() < 2);
    let (mut sqrt, mut r_lo) = sqrt_rem_newton::<Limb, SignedLimb>(n_hi);
    const PREC: u64 = Limb::WIDTH >> 1;
    const PREC_P_1: u64 = PREC + 1;
    const PREC_M_1: u64 = PREC - 1;
    // r_lo <= 2 * sqrt < 2 ^ (prec + 1)
    r_lo = (r_lo << PREC_M_1) | (n_lo >> PREC_P_1);
    let mut q = r_lo / sqrt;
    // q <= 2 ^ prec, if q = 2 ^ prec, reduce the overestimate.
    q -= q >> PREC;
    // now we have q < 2 ^ prec
    let u = r_lo - q * sqrt;
    // now we have (rp_lo << prec + n_lo >> prec) / 2 = q * sqrt + u
    sqrt = (sqrt << PREC) | q;
    let mut r_hi = (u >> PREC_M_1) + 1;
    r_lo = (u << PREC_P_1) | (n_lo.mod_power_of_2(PREC_P_1));
    let q_squared = q.square();
    if r_lo < q_squared {
        assert_ne!(r_hi, 0);
        r_hi -= 1;
    }
    r_lo.wrapping_sub_assign(q_squared);
    if r_hi == 0 {
        r_lo.wrapping_add_assign(sqrt);
        if r_lo < sqrt {
            r_hi += 1;
        }
        sqrt -= 1;
        r_lo.wrapping_add_assign(sqrt);
        if r_lo < sqrt {
            r_hi += 1;
        }
    }
    r_hi -= 1;
    assert!(r_hi < 2);
    (sqrt, r_hi == 1, r_lo)
}

pub const fn _limbs_sqrt_rem_helper_scratch_len(n: usize) -> usize {
    (n >> 1) + 1
}

/// Let n be out.len().
/// Let x be xs[..2 * n] before execution.
/// Let s be out after execution.
/// Let r be xs[..n] after execution.
/// 
/// xs[2 * n - 1].leading_zeros() must be less than 2.
/// 
/// If approx = 0, then s = floor(sqrt(x)) and r = x - s ^ 2.
///
/// This is mpn_dc_sqrtrem from mpn/generic/sqrtrem.c, GMP 6.2.1.
#[doc(hidden)]
pub fn _limbs_sqrt_rem_helper(
    out: &mut [Limb],
    xs: &mut [Limb],
    approx: Limb,
    scratch: &mut [Limb],
) -> bool {
    let n = out.len();
    assert!(n > 1);
    let xs = &mut xs[..n << 1];
    assert!(xs.last().unwrap().leading_zeros() < 2);
    let h1 = n >> 1;
    let h2 = n - h1;
    let two_h1 = h1 << 1;
    let xs_hi = &mut xs[two_h1..];
    let out_hi = &mut out[h1..];
    let q = if h2 == 1 {
        let (sqrt, r_hi, r_lo) = _sqrt_rem_2_newton(xs_hi[1], xs_hi[0]);
        out_hi[0] = sqrt;
        xs_hi[0] = r_lo;
        r_hi
    } else {
        _limbs_sqrt_rem_helper(out_hi, xs_hi, 0, scratch)
    };
    if q {
        assert!(limbs_sub_same_length_in_place_left(
            &mut xs_hi[..h2],
            out_hi
        ));
    }
    let xs_hi = &mut xs[h1..];
    if h2 == 1 {
        xs_hi[0] = limbs_div_limb_to_out_mod(scratch, &xs_hi[..n], out_hi[0]);
    } else {
        limbs_div_mod_qs_to_out_rs_to_ns(scratch, &mut xs_hi[..n], out_hi);
    }
    let mut q = Limb::iverson(q);
    q += scratch[h1];
    let mut r_hi = scratch[0].odd();
    limbs_shr_to_out(out, &scratch[..h1], 1);
    out[h1 - 1] |= q << (Limb::WIDTH - 1);
    if (out[0] & approx) != 0 {
        return true;
    }
    q >>= 1;
    let (out_lo, out_hi) = out.split_at_mut(h1);
    if r_hi {
        r_hi = limbs_slice_add_same_length_in_place_left(&mut xs_hi[..h2], out_hi);
    }
    let (xs, xs_hi_hi) = xs.split_at_mut(n);
    let (xs_lo, xs_hi) = xs.split_at_mut(two_h1);
    limbs_square_to_out(xs_hi_hi, out_lo);
    let mut b = q;
    if limbs_sub_same_length_in_place_left(xs_lo, &xs_hi_hi[..two_h1]) {
        b += 1;
    }
    let mut r_hi = SignedLimb::iverson(r_hi);
    r_hi -= if h1 == h2 {
        SignedLimb::exact_from(b)
    } else {
        SignedLimb::iverson(limbs_sub_limb_in_place(xs_hi, b))
    };
    if r_hi < 0 {
        q = Limb::iverson(limbs_slice_add_limb_in_place(out_hi, q));
        r_hi += SignedLimb::exact_from(
            limbs_slice_add_mul_limb_same_length_in_place_left(xs, out, 2) + (q << 1),
        );
        if limbs_sub_limb_in_place(xs, 1) {
            r_hi -= 1;
        }
        limbs_sub_limb_in_place(out, 1);
    }
    assert!(r_hi >= 0);
    assert!(r_hi < 2);
    r_hi == 1
}

/// This is mpn_divappr_q from mpn/generic/sqrtrem.c, GMP 6.2.1.
fn limbs_div_approx_helper(qs: &mut [Limb], ns: &[Limb], ds: &[Limb], scratch: &mut [Limb]) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(d_len > 2);
    assert!(n_len >= d_len);
    assert!(ds.last().unwrap().get_highest_bit());
    let scratch = &mut scratch[..n_len];
    scratch.copy_from_slice(ns);
    let inv = limbs_two_limb_inverse_helper(ds[d_len - 1], ds[d_len - 2]);
    qs[n_len - d_len] = Limb::iverson(if d_len < DC_DIVAPPR_Q_THRESHOLD {
        _limbs_div_schoolbook_approx(qs, scratch, ds, inv)
    } else if d_len < MU_DIVAPPR_Q_THRESHOLD {
        _limbs_div_divide_and_conquer_approx(qs, scratch, ds, inv)
    } else {
        let mut new_scratch = vec![0; _limbs_div_barrett_approx_scratch_len(n_len, d_len)];
        _limbs_div_barrett_approx(qs, ns, ds, &mut new_scratch)
    });
}

/// Let n be out.len().
/// Let m be xs.len().
/// n must be ceiling(m / 2).
/// odd must be m.odd().
/// shift must be floor(xs[m - 1].leading_zeros() / 2).
/// Let x be xs before execution.
/// Let s be out after execution.
/// Then s = floor(sqrt(x)).
/// The return value is true iff there is a remainder (that is, x is not a perfect square).
///
/// This is mpn_dc_sqrt from mpn/generic/sqrtrem.c, GMP 6.2.1.
pub fn _limbs_sqrt_helper(out: &mut [Limb], xs: &[Limb], shift: u64, odd: bool) -> bool {
    let n = out.len();
    let odd = usize::iverson(odd);
    assert_eq!(xs.len(), 2 * n - odd);
    assert_ne!(xs[2 * n - 1 - odd], 0);
    assert!(n > 4);
    assert!(shift < Limb::WIDTH >> 1);
    let h1 = (n - 1) >> 1;
    let h2 = n - h1;
    let (out_lo, out_hi) = out.split_at_mut(h1);
    let mut scratch = vec![0; (n << 1) + h1 + 4];
    let scratch_hi = &mut scratch[n..]; // length is n + h1 + 4
    if shift != 0 {
        // o is used to exactly set the lowest bits of the dividend.
        let o = usize::iverson(h1 > (1 + odd));
        assert_eq!(
            limbs_shl_to_out(
                &mut scratch_hi[1 - o..],
                &xs[h1 - 1 - o - odd..(n << 1) - odd],
                shift << 1
            ),
            0
        );
    } else {
        scratch_hi[1..n + h2 + 2].copy_from_slice(&xs[h1 - 1 - odd..(n << 1) - odd]);
    }
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(n + 1); // scratch_hi len is n + h1 + 3
    let r_hi = _limbs_sqrt_rem_helper(out_hi, &mut scratch_hi[h1 + 1..n + h2 + 1], 0, scratch_lo);
    if r_hi {
        assert!(limbs_sub_same_length_in_place_left(
            &mut scratch_hi[h1 + 1..n + 1],
            out_hi
        ));
    }
    // qs len is h1 + 2
    let (scratch_hi_lo, qs) = scratch_hi.split_at_mut(n + 1);
    limbs_div_approx_helper(qs, scratch_hi_lo, out_hi, scratch_lo);
    // qs_tail len is h1 + 1
    let (qs_head, qs_tail) = qs.split_first_mut().unwrap();
    let mut qs_last = Limb::iverson(r_hi);
    qs_last += qs_tail[h1];
    let mut nonzero_remainder = true;
    if qs_last > 1 {
        for x in out_lo {
            *x = Limb::MAX;
        }
    } else {
        limbs_shr_to_out(out_lo, &qs_tail[..h1], 1);
        if qs_last != 0 {
            out_lo.last_mut().unwrap().set_bit(Limb::WIDTH - 1);
        }
        let s = (Limb::WIDTH >> odd) - shift - 1;
        if (*qs_head >> 3) | qs_tail[0].mod_power_of_2(Limb::WIDTH - s) == 0 {
            // Approximation is not good enough, the extra limb(+ shift bits)
            // is smaller than needed to absorb the possible error.
            // {qs + 1, h1 + 1} equals 2*{out, h1}
            assert_eq!(limbs_mul_greater_to_out(scratch_lo, out_hi, qs_tail), 0);
            // scratch_hi_1 len is n + h1 + 2
            let scratch_hi_1 = &mut scratch_hi[1..];
            // scratch_lo_hi len is h1 + 1
            let (scratch_lo_lo, scratch_lo_hi) = scratch_lo.split_at_mut(h2);
            // scratch_hi_1_hi len is 2 * h1 + 2
            let (scratch_hi_1_lo, scratch_hi_1_hi) = scratch_hi_1.split_at_mut(h2);
            // Compute the remainder of the previous mpn_div(appr)_q.
            if limbs_sub_same_length_in_place_left(scratch_hi_1_lo, scratch_lo_lo) {
                assert!(!limbs_sub_limb_in_place(&mut scratch_hi_1_hi[..h1], 1));
            }
            let cmp = limbs_cmp_same_length(&scratch_hi_1_hi[..h1], &scratch_lo_hi[..h1]);
            assert_ne!(cmp, Ordering::Greater);
            if cmp == Ordering::Less {
                // May happen only if div result was not exact.
                let carry =
                    limbs_slice_add_mul_limb_same_length_in_place_left(scratch_hi_1_lo, out_hi, 2);
                assert!(!limbs_slice_add_limb_in_place(
                    &mut scratch_hi_1_hi[..h1],
                    carry
                ));
                assert!(!limbs_sub_limb_in_place(out_lo, 1));
            }
            // scratch_hi_1_hi len is 2 * h1 + 2
            let (scratch_hi_1_lo, scratch_hi_1_hi) = scratch_hi_1.split_at_mut(h1);
            if slice_test_zero(&scratch_hi_1_hi[..h2 - h1]) {
                limbs_square_to_out(scratch_lo, out_lo);
                // scratch_lo_hi len is h2 + 1
                let (scratch_lo_lo, scratch_lo_hi) = scratch_lo.split_at(h1);
                let mut cmp = limbs_cmp_same_length(scratch_hi_1_lo, &scratch_lo_hi[..h1]);
                if cmp == Ordering::Equal {
                    let scratch = &scratch_lo_lo[odd..];
                    cmp = if shift != 0 {
                        limbs_shl_to_out(scratch_hi, &xs[..h1], shift << 1);
                        limbs_cmp_same_length(&scratch_hi[..h1 - odd], scratch)
                    } else {
                        limbs_cmp_same_length(&xs[..h1 - odd], scratch)
                    };
                }
                if cmp == Ordering::Less {
                    assert!(!limbs_sub_limb_in_place(out_lo, 1));
                }
                nonzero_remainder = cmp != Ordering::Equal;
            }
        }
    }
    if odd == 1 || shift != 0 {
        let mut shift = shift;
        if odd == 1 {
            shift.set_bit(Limb::LOG_WIDTH - 1);
        }
        limbs_slice_shr_in_place(out, shift);
    }
    nonzero_remainder
}

fn floor_inverse_binary<F: Fn(&Natural) -> Natural>(
    f: F,
    x: &Natural,
    mut low: Natural,
    mut high: Natural,
) -> Natural {
    loop {
        if high <= low {
            return low;
        }
        let mid = (&low + &high).shr_round(1, RoundingMode::Ceiling);
        match f(&mid).cmp(x) {
            Ordering::Equal => return mid,
            Ordering::Less => low = mid,
            Ordering::Greater => high = mid - Natural::ONE,
        }
    }
}

#[doc(hidden)]
pub fn _floor_sqrt_binary(x: &Natural) -> Natural {
    if x < &Natural::TWO {
        x.clone()
    } else {
        let p = Natural::power_of_2(x.significant_bits().shr_round(1, RoundingMode::Ceiling));
        floor_inverse_binary(|x| x.square(), x, &p >> 1, p)
    }
}

#[doc(hidden)]
pub fn _ceiling_sqrt_binary(x: &Natural) -> Natural {
    let floor_sqrt = _floor_sqrt_binary(x);
    if &(&floor_sqrt).square() == x {
        floor_sqrt
    } else {
        floor_sqrt + Natural::ONE
    }
}

#[doc(hidden)]
pub fn _checked_sqrt_binary(x: &Natural) -> Option<Natural> {
    let floor_sqrt = _floor_sqrt_binary(x);
    if &(&floor_sqrt).square() == x {
        Some(floor_sqrt)
    } else {
        None
    }
}

#[doc(hidden)]
pub fn _sqrt_rem_binary(x: &Natural) -> (Natural, Natural) {
    let floor_sqrt = _floor_sqrt_binary(x);
    let rem = x - (&floor_sqrt).square();
    (floor_sqrt, rem)
}

//TODO use better algorithms

impl FloorSqrtAssign for Natural {
    /// Replaces a `Natural` with the floor of its square root.
    ///
    /// $x \gets \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::FloorSqrtAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(99u8);
    /// x.floor_sqrt_assign();
    /// assert_eq!(x, 9);
    ///
    /// let mut x = Natural::from(100u8);
    /// x.floor_sqrt_assign();
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(101u8);
    /// x.floor_sqrt_assign();
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(1000000000u32);
    /// x.floor_sqrt_assign();
    /// assert_eq!(x, 31622);
    ///
    /// let mut x = Natural::from(10000000000u64);
    /// x.floor_sqrt_assign();
    /// assert_eq!(x, 100000);
    /// ```
    #[inline]
    fn floor_sqrt_assign(&mut self) {
        *self = _floor_sqrt_binary(&*self);
    }
}

impl FloorSqrt for Natural {
    type Output = Natural;

    /// Returns the floor of the square root of a `Natural`, taking the `Natural` by value.
    ///
    /// $f(x) = \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::FloorSqrt;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(99u8).floor_sqrt(), 9);
    /// assert_eq!(Natural::from(100u8).floor_sqrt(), 10);
    /// assert_eq!(Natural::from(101u8).floor_sqrt(), 10);
    /// assert_eq!(Natural::from(1000000000u32).floor_sqrt(), 31622);
    /// assert_eq!(Natural::from(10000000000u64).floor_sqrt(), 100000);
    /// ```
    #[inline]
    fn floor_sqrt(self) -> Natural {
        _floor_sqrt_binary(&self)
    }
}

impl<'a> FloorSqrt for &'a Natural {
    type Output = Natural;

    /// Returns the floor of the square root of a `Natural`, taking the `Natural` by reference.
    ///
    /// $f(x) = \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::FloorSqrt;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(99u8)).floor_sqrt(), 9);
    /// assert_eq!((&Natural::from(100u8)).floor_sqrt(), 10);
    /// assert_eq!((&Natural::from(101u8)).floor_sqrt(), 10);
    /// assert_eq!((&Natural::from(1000000000u32)).floor_sqrt(), 31622);
    /// assert_eq!((&Natural::from(10000000000u64)).floor_sqrt(), 100000);
    /// ```
    #[inline]
    fn floor_sqrt(self) -> Natural {
        _floor_sqrt_binary(self)
    }
}

impl CeilingSqrtAssign for Natural {
    /// Replaces a `Natural` with the ceiling of its square root.
    ///
    /// $x \gets \lceil\sqrt{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingSqrtAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(99u8);
    /// x.ceiling_sqrt_assign();
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(100u8);
    /// x.ceiling_sqrt_assign();
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(101u8);
    /// x.ceiling_sqrt_assign();
    /// assert_eq!(x, 11);
    ///
    /// let mut x = Natural::from(1000000000u32);
    /// x.ceiling_sqrt_assign();
    /// assert_eq!(x, 31623);
    ///
    /// let mut x = Natural::from(10000000000u64);
    /// x.ceiling_sqrt_assign();
    /// assert_eq!(x, 100000);
    /// ```
    #[inline]
    fn ceiling_sqrt_assign(&mut self) {
        *self = _ceiling_sqrt_binary(&*self);
    }
}

impl CeilingSqrt for Natural {
    type Output = Natural;

    /// Returns the ceiling of the square root of a `Natural`, taking the `Natural` by value.
    ///
    /// $f(x) = \lceil\sqrt{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingSqrt;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(99u8).ceiling_sqrt(), 10);
    /// assert_eq!(Natural::from(100u8).ceiling_sqrt(), 10);
    /// assert_eq!(Natural::from(101u8).ceiling_sqrt(), 11);
    /// assert_eq!(Natural::from(1000000000u32).ceiling_sqrt(), 31623);
    /// assert_eq!(Natural::from(10000000000u64).ceiling_sqrt(), 100000);
    /// ```
    #[inline]
    fn ceiling_sqrt(self) -> Natural {
        _ceiling_sqrt_binary(&self)
    }
}

impl<'a> CeilingSqrt for &'a Natural {
    type Output = Natural;

    /// Returns the ceiling of the square root of a `Natural`, taking the `Natural` by reference.
    ///
    /// $f(x) = \lceil\sqrt{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingSqrt;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(99u8).ceiling_sqrt(), 10);
    /// assert_eq!(Natural::from(100u8).ceiling_sqrt(), 10);
    /// assert_eq!(Natural::from(101u8).ceiling_sqrt(), 11);
    /// assert_eq!(Natural::from(1000000000u32).ceiling_sqrt(), 31623);
    /// assert_eq!(Natural::from(10000000000u64).ceiling_sqrt(), 100000);
    /// ```
    #[inline]
    fn ceiling_sqrt(self) -> Natural {
        _ceiling_sqrt_binary(self)
    }
}

impl CheckedSqrt for Natural {
    type Output = Natural;

    /// Returns the the square root of a `Natural`, or `None` if the `Natural` is not a perfect
    /// square. The `Natural` is taken by value.
    ///
    /// $$
    /// f(x) = \\begin{cases}
    ///     \operatorname{Some}(sqrt{x}) & \sqrt{x} \in \Z \\\\
    ///     \operatorname{None} & \textrm{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedSqrt;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(99u8).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!(Natural::from(100u8).checked_sqrt().to_debug_string(), "Some(10)");
    /// assert_eq!(Natural::from(101u8).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!(Natural::from(1000000000u32).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!(Natural::from(10000000000u64).checked_sqrt().to_debug_string(), "Some(100000)");
    /// ```
    #[inline]
    fn checked_sqrt(self) -> Option<Natural> {
        _checked_sqrt_binary(&self)
    }
}

impl<'a> CheckedSqrt for &'a Natural {
    type Output = Natural;

    /// Returns the the square root of a `Natural`, or `None` if the `Natural` is not a perfect
    /// square. The `Natural` is taken by reference.
    ///
    /// $$
    /// f(x) = \\begin{cases}
    ///     \operatorname{Some}(sqrt{x}) & \sqrt{x} \in \Z \\\\
    ///     \operatorname{None} & \textrm{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedSqrt;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(99u8)).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!((&Natural::from(100u8)).checked_sqrt().to_debug_string(), "Some(10)");
    /// assert_eq!((&Natural::from(101u8)).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!((&Natural::from(1000000000u32)).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!(
    ///     (&Natural::from(10000000000u64)).checked_sqrt().to_debug_string(),
    ///     "Some(100000)"
    /// );
    /// ```
    #[inline]
    fn checked_sqrt(self) -> Option<Natural> {
        _checked_sqrt_binary(self)
    }
}

impl SqrtRemAssign for Natural {
    type RemOutput = Natural;

    /// Replaces a `Natural` with the floor of its square root, and returns the remainder (the
    /// difference between the original `Natural` and the square of the floor).
    ///
    /// $f(x) = x - \lfloor\sqrt{x}\rfloor^2$,
    ///
    /// $x \gets \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SqrtRemAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(99u8);
    /// assert_eq!(x.sqrt_rem_assign(), 18);
    /// assert_eq!(x, 9);
    ///
    /// let mut x = Natural::from(100u8);
    /// assert_eq!(x.sqrt_rem_assign(), 0);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(101u8);
    /// assert_eq!(x.sqrt_rem_assign(), 1);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(1000000000u32);
    /// assert_eq!(x.sqrt_rem_assign(), 49116);
    /// assert_eq!(x, 31622);
    ///
    /// let mut x = Natural::from(10000000000u64);
    /// assert_eq!(x.sqrt_rem_assign(), 0);
    /// assert_eq!(x, 100000);
    /// ```
    #[inline]
    fn sqrt_rem_assign(&mut self) -> Natural {
        let (sqrt, rem) = _sqrt_rem_binary(&*self);
        *self = sqrt;
        rem
    }
}

impl SqrtRem for Natural {
    type SqrtOutput = Natural;
    type RemOutput = Natural;

    /// Returns the floor of the square root of a `Natural`, and the remainder (the difference
    /// between the `Natural` and the square of the floor). The `Natural` is taken by value.
    ///
    /// $f(x) = (\lfloor\sqrt{x}\rfloor, x - \lfloor\sqrt{x}\rfloor^2)$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SqrtRem;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(99u8).sqrt_rem().to_debug_string(), "(9, 18)");
    /// assert_eq!(Natural::from(100u8).sqrt_rem().to_debug_string(), "(10, 0)");
    /// assert_eq!(Natural::from(101u8).sqrt_rem().to_debug_string(), "(10, 1)");
    /// assert_eq!(Natural::from(1000000000u32).sqrt_rem().to_debug_string(), "(31622, 49116)");
    /// assert_eq!(Natural::from(10000000000u64).sqrt_rem().to_debug_string(), "(100000, 0)");
    /// ```
    #[inline]
    fn sqrt_rem(self) -> (Natural, Natural) {
        _sqrt_rem_binary(&self)
    }
}

impl<'a> SqrtRem for &'a Natural {
    type SqrtOutput = Natural;
    type RemOutput = Natural;

    /// Returns the floor of the square root of a `Natural`, and the remainder (the difference
    /// between the `Natural` and the square of the floor). The `Natural` is taken by reference.
    ///
    /// $f(x) = (\lfloor\sqrt{x}\rfloor, x - \lfloor\sqrt{x}\rfloor^2)$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SqrtRem;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(99u8)).sqrt_rem().to_debug_string(), "(9, 18)");
    /// assert_eq!((&Natural::from(100u8)).sqrt_rem().to_debug_string(), "(10, 0)");
    /// assert_eq!((&Natural::from(101u8)).sqrt_rem().to_debug_string(), "(10, 1)");
    /// assert_eq!((&Natural::from(1000000000u32)).sqrt_rem().to_debug_string(), "(31622, 49116)");
    /// assert_eq!((&Natural::from(10000000000u64)).sqrt_rem().to_debug_string(), "(100000, 0)");
    /// ```
    #[inline]
    fn sqrt_rem(self) -> (Natural, Natural) {
        _sqrt_rem_binary(self)
    }
}

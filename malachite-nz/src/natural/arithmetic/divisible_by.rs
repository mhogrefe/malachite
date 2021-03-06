use malachite_base::num::arithmetic::traits::{DivisibleBy, DivisibleByPowerOf2, Parity};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_base::slices::{slice_leading_zeros, slice_test_zero};
use natural::arithmetic::div_exact::{
    _limbs_modular_div_mod_barrett, _limbs_modular_div_mod_barrett_scratch_len,
    _limbs_modular_div_mod_divide_and_conquer, _limbs_modular_div_mod_schoolbook,
    limbs_modular_invert_limb,
};
use natural::arithmetic::eq_mod::_limbs_mod_exact_odd_limb;
use natural::arithmetic::mod_op::limbs_mod_limb;
use natural::arithmetic::shr::{limbs_shr_to_out, limbs_slice_shr_in_place};
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::{Limb, BMOD_1_TO_MOD_1_THRESHOLD, DC_BDIV_QR_THRESHOLD, MU_BDIV_QR_THRESHOLD};

/// Interpreting a slice of `Limb`s as the limbs of a `Natural` in ascending order, determines
/// whether that `Natural` is divisible by a given limb.
///
/// This function assumes that `ns` has at least two elements and that `d` is nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `ns.len()`
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::divisible_by::limbs_divisible_by_limb;
///
/// assert_eq!(limbs_divisible_by_limb(&[333, 333], 3), true);
/// assert_eq!(limbs_divisible_by_limb(&[332, 333], 3), false);
/// ```
///
/// This is mpz_divisible_ui_p from mpz/divis_ui.c, GMP 6.2.1, where a is non-negative and the
/// ABOVE_THRESHOLD branch is excluded.
pub fn limbs_divisible_by_limb(ns: &[Limb], d: Limb) -> bool {
    assert!(ns.len() > 1);
    if d.even() {
        let twos = TrailingZeros::trailing_zeros(d);
        ns[0].divisible_by_power_of_2(twos) && _limbs_mod_exact_odd_limb(ns, d >> twos, 0) == 0
    } else {
        _limbs_mod_exact_odd_limb(ns, d, 0) == 0
    }
}

/// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
/// `Natural`s, determines whether the first `Natural` is divisible by the second. Both `Natural`s
/// are taken by value.
///
/// `ns` must be at least as long as `ds`, both slices must be nonempty, and the last limb of both
/// must be nonzero.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `ns.len()`
///
/// # Panics
/// Panics if `ns` is shorter than `ds`, `ds` is empty, or the last limbs of either slice are zero.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::divisible_by::limbs_divisible_by;
///
/// assert!(!limbs_divisible_by(&mut [1, 2, 3], &mut [4, 5]));
/// assert!(
///     limbs_divisible_by(&mut [10200, 20402, 30605, 20402, 10200], &mut [100, 101, 102])
/// );
/// ```
///
/// This is mpn_divisible_p from mpn/generic/divis.c, GMP 6.2.1, where an >= dn and neither are
/// zero.
#[allow(clippy::absurd_extreme_comparisons)]
pub fn limbs_divisible_by(ns: &mut [Limb], ds: &mut [Limb]) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert_ne!(*ns.last().unwrap(), 0);
    assert_ne!(*ds.last().unwrap(), 0);
    // Strip low zero limbs from ds, requiring n == 0 on those.
    let offset = slice_leading_zeros(ds);
    let (ns_lo, ns) = ns.split_at_mut(offset);
    if !slice_test_zero(ns_lo) {
        // n has fewer low zero limbs than d, so not divisible
        return false;
    }
    let n_len = ns.len();
    let ds = &mut ds[offset..];
    let d_len = ds.len();
    let n_0 = ns[0];
    let d_0 = ds[0];
    // n must have at least as many low zero bits as d
    let d_mask = (d_0 & d_0.wrapping_neg()).wrapping_sub(1);
    if n_0 & d_mask != 0 {
        return false;
    }
    if d_len == 1 {
        return if n_len >= BMOD_1_TO_MOD_1_THRESHOLD {
            limbs_mod_limb(ns, d_0) == 0
        } else {
            _limbs_mod_exact_odd_limb(ns, d_0 >> d_0.trailing_zeros(), 0) == 0
        };
    }
    let trailing_zeros = TrailingZeros::trailing_zeros(d_0);
    if d_len == 2 {
        let d_1 = ds[1];
        if d_1 <= d_mask {
            let d_low = (d_0 >> trailing_zeros) | (d_1 << (Limb::WIDTH - trailing_zeros));
            return if n_len < BMOD_1_TO_MOD_1_THRESHOLD {
                _limbs_mod_exact_odd_limb(ns, d_low, 0)
            } else {
                limbs_mod_limb(ns, d_low)
            } == 0;
        }
    }
    let n_len_plus_1 = n_len + 1;
    let mut qs = vec![0; n_len_plus_1 - d_len];
    if trailing_zeros != 0 {
        assert_eq!(limbs_slice_shr_in_place(ds, trailing_zeros), 0);
        assert_eq!(limbs_slice_shr_in_place(ns, trailing_zeros), 0);
    }
    let mut rs_vec;
    let rs = if ns[n_len - 1] >= ds[d_len - 1] {
        rs_vec = Vec::with_capacity(n_len_plus_1);
        rs_vec.extend_from_slice(ns);
        rs_vec.push(0);
        &mut rs_vec
    } else if n_len == d_len {
        return false;
    } else {
        ns
    };
    let r_len = rs.len();
    let q_len = r_len - d_len;
    let rs = if d_len < DC_BDIV_QR_THRESHOLD || q_len < DC_BDIV_QR_THRESHOLD {
        let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_mod_schoolbook(&mut qs, rs, ds, d_inv);
        &mut rs[q_len..]
    } else if d_len < MU_BDIV_QR_THRESHOLD {
        let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_mod_divide_and_conquer(&mut qs, rs, ds, d_inv);
        &mut rs[q_len..]
    } else {
        let mut scratch = vec![0; _limbs_modular_div_mod_barrett_scratch_len(r_len, d_len)];
        let ns = rs.to_vec();
        _limbs_modular_div_mod_barrett(&mut qs, rs, &ns, ds, &mut scratch);
        &mut rs[..d_len]
    };
    slice_test_zero(rs)
}

/// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
/// `Natural`s, determines whether the first `Natural` is divisible by the second. The first
/// `Natural` is taken by value, and the second by reference.
///
/// `ns` must be at least as long as `ds`, both slices must be nonempty, and the last limb of both
/// must be nonzero.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `ns.len()`
///
/// # Panics
/// Panics if `ns` is shorter than `ds`, `ds` is empty, or the last limbs of either slice are zero.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::divisible_by::limbs_divisible_by_val_ref;
///
/// assert!(!limbs_divisible_by_val_ref(&mut [1, 2, 3], &[4, 5]));
/// assert!(
///     limbs_divisible_by_val_ref(&mut [10200, 20402, 30605, 20402, 10200], &[100, 101, 102])
/// );
/// ```
///
/// This is mpn_divisible_p from mpn/generic/divis.c, GMP 6.2.1, where an >= dn and neither are
/// zero.
#[allow(clippy::absurd_extreme_comparisons)]
pub fn limbs_divisible_by_val_ref(ns: &mut [Limb], ds: &[Limb]) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert_ne!(*ns.last().unwrap(), 0);
    assert_ne!(*ds.last().unwrap(), 0);
    // Strip low zero limbs from ds, requiring n == 0 on those.
    let offset = slice_leading_zeros(ds);
    let (ns_lo, ns) = ns.split_at_mut(offset);
    if !slice_test_zero(ns_lo) {
        // n has fewer low zero limbs than d, so not divisible
        return false;
    }
    let n_len = ns.len();
    let mut scratch;
    let mut ds = &ds[offset..];
    let d_len = ds.len();
    let n_0 = ns[0];
    let d_0 = ds[0];
    // n must have at least as many low zero bits as d
    let d_mask = (d_0 & d_0.wrapping_neg()).wrapping_sub(1);
    if n_0 & d_mask != 0 {
        return false;
    }
    if d_len == 1 {
        return if n_len >= BMOD_1_TO_MOD_1_THRESHOLD {
            limbs_mod_limb(ns, d_0) == 0
        } else {
            _limbs_mod_exact_odd_limb(ns, d_0 >> d_0.trailing_zeros(), 0) == 0
        };
    }
    let trailing_zeros = TrailingZeros::trailing_zeros(d_0);
    if d_len == 2 {
        let d_1 = ds[1];
        if d_1 <= d_mask {
            let d_low = (d_0 >> trailing_zeros) | (d_1 << (Limb::WIDTH - trailing_zeros));
            return if n_len < BMOD_1_TO_MOD_1_THRESHOLD {
                _limbs_mod_exact_odd_limb(ns, d_low, 0)
            } else {
                limbs_mod_limb(ns, d_low)
            } == 0;
        }
    }
    let n_len_plus_1 = n_len + 1;
    let mut qs = vec![0; n_len_plus_1 - d_len];
    if trailing_zeros != 0 {
        scratch = vec![0; d_len];
        assert_eq!(limbs_shr_to_out(&mut scratch, ds, trailing_zeros), 0);
        ds = &scratch;
        assert_eq!(limbs_slice_shr_in_place(ns, trailing_zeros), 0);
    }
    let mut rs_vec;
    let rs = if ns[n_len - 1] >= ds[d_len - 1] {
        rs_vec = Vec::with_capacity(n_len_plus_1);
        rs_vec.extend_from_slice(ns);
        rs_vec.push(0);
        &mut rs_vec
    } else if n_len == d_len {
        return false;
    } else {
        ns
    };
    let r_len = rs.len();
    let q_len = r_len - d_len;
    let rs = if d_len < DC_BDIV_QR_THRESHOLD || q_len < DC_BDIV_QR_THRESHOLD {
        let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_mod_schoolbook(&mut qs, rs, ds, d_inv);
        &mut rs[q_len..]
    } else if d_len < MU_BDIV_QR_THRESHOLD {
        let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_mod_divide_and_conquer(&mut qs, rs, ds, d_inv);
        &mut rs[q_len..]
    } else {
        let mut scratch = vec![0; _limbs_modular_div_mod_barrett_scratch_len(r_len, d_len)];
        let ns = rs.to_vec();
        _limbs_modular_div_mod_barrett(&mut qs, rs, &ns, ds, &mut scratch);
        &mut rs[..d_len]
    };
    slice_test_zero(rs)
}

/// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
/// `Natural`s, determines whether the first `Natural` is divisible by the second. The first
/// `Natural` is taken by reference, and the second by value.
///
/// `ns` must be at least as long as `ds`, both slices must be nonempty, and the last limb of both
/// must be nonzero.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `ns.len()`
///
/// # Panics
/// Panics if `ns` is shorter than `ds`, `ds` is empty, or the last limbs of either slice are zero.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::divisible_by::limbs_divisible_by_ref_val;
///
/// assert!(!limbs_divisible_by_ref_val(&[1, 2, 3], &mut [4, 5]));
/// assert!(
///     limbs_divisible_by_ref_val(&[10200, 20402, 30605, 20402, 10200], &mut [100, 101, 102])
/// );
/// ```
///
/// This is mpn_divisible_p from mpn/generic/divis.c, GMP 6.2.1, where an >= dn and neither are
/// zero.
#[allow(clippy::absurd_extreme_comparisons)]
pub fn limbs_divisible_by_ref_val(ns: &[Limb], ds: &mut [Limb]) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert_ne!(*ns.last().unwrap(), 0);
    assert_ne!(*ds.last().unwrap(), 0);
    // Strip low zero limbs from ds, requiring n == 0 on those.
    let offset = slice_leading_zeros(ds);
    let (ns_lo, ns) = ns.split_at(offset);
    if !slice_test_zero(ns_lo) {
        // n has fewer low zero limbs than d, so not divisible
        return false;
    }
    let n_len = ns.len();
    let ds = &mut ds[offset..];
    let d_len = ds.len();
    let n_0 = ns[0];
    let d_0 = ds[0];
    // n must have at least as many low zero bits as d
    let d_mask = (d_0 & d_0.wrapping_neg()).wrapping_sub(1);
    if n_0 & d_mask != 0 {
        return false;
    }
    if d_len == 1 {
        return if n_len >= BMOD_1_TO_MOD_1_THRESHOLD {
            limbs_mod_limb(ns, d_0) == 0
        } else {
            _limbs_mod_exact_odd_limb(ns, d_0 >> d_0.trailing_zeros(), 0) == 0
        };
    }
    let trailing_zeros = TrailingZeros::trailing_zeros(d_0);
    if d_len == 2 {
        let d_1 = ds[1];
        if d_1 <= d_mask {
            let d_low = (d_0 >> trailing_zeros) | (d_1 << (Limb::WIDTH - trailing_zeros));
            return if n_len < BMOD_1_TO_MOD_1_THRESHOLD {
                _limbs_mod_exact_odd_limb(ns, d_low, 0)
            } else {
                limbs_mod_limb(ns, d_low)
            } == 0;
        }
    }
    let n_len_plus_1 = n_len + 1;
    let mut rs_qs = vec![0; (n_len_plus_1 << 1) - d_len];
    let (rs, qs) = rs_qs.split_at_mut(n_len_plus_1);
    if trailing_zeros != 0 {
        assert_eq!(limbs_slice_shr_in_place(ds, trailing_zeros), 0);
        assert_eq!(limbs_shr_to_out(rs, ns, trailing_zeros), 0);
    } else {
        rs[..n_len].copy_from_slice(ns);
    }
    let r_len = if rs[n_len - 1] >= ds[d_len - 1] {
        n_len_plus_1
    } else if n_len == d_len {
        return false;
    } else {
        n_len
    };
    let rs = &mut rs[..r_len];
    let q_len = r_len - d_len;
    let rs = if d_len < DC_BDIV_QR_THRESHOLD || q_len < DC_BDIV_QR_THRESHOLD {
        let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_mod_schoolbook(qs, rs, ds, d_inv);
        &mut rs[q_len..]
    } else if d_len < MU_BDIV_QR_THRESHOLD {
        let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_mod_divide_and_conquer(qs, rs, ds, d_inv);
        &mut rs[q_len..]
    } else {
        let mut scratch = vec![0; _limbs_modular_div_mod_barrett_scratch_len(r_len, d_len)];
        let ns = rs.to_vec();
        _limbs_modular_div_mod_barrett(qs, rs, &ns, ds, &mut scratch);
        &mut rs[..d_len]
    };
    slice_test_zero(rs)
}

/// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
/// `Natural`s, determines whether the first `Natural` is divisible by the second. Both `Natural`s
///// are taken by reference.
///
/// `ns` must be at least as long as `ds`, both slices must be nonempty, and the last limb of both
/// must be nonzero.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `ns.len()`
///
/// # Panics
/// Panics if `ns` is shorter than `ds`, `ds` is empty, or the last limbs of either slice are zero.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::divisible_by::limbs_divisible_by_ref_ref;
///
/// assert!(!limbs_divisible_by_ref_ref(&[1, 2, 3], &[4, 5]));
/// assert!(
///     limbs_divisible_by_ref_ref(&[10200, 20402, 30605, 20402, 10200], &[100, 101, 102])
/// );
/// ```
///
/// This is mpn_divisible_p from mpn/generic/divis.c, GMP 6.2.1, where an >= dn and neither are
/// zero.
#[allow(clippy::absurd_extreme_comparisons)]
pub fn limbs_divisible_by_ref_ref(ns: &[Limb], ds: &[Limb]) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert_ne!(*ns.last().unwrap(), 0);
    assert_ne!(*ds.last().unwrap(), 0);
    // Strip low zero limbs from ds, requiring n == 0 on those.
    let offset = slice_leading_zeros(ds);
    let (ns_lo, ns) = ns.split_at(offset);
    if !slice_test_zero(ns_lo) {
        // n has fewer low zero limbs than d, so not divisible
        return false;
    }
    let n_len = ns.len();
    let mut scratch;
    let mut ds = &ds[offset..];
    let d_len = ds.len();
    let d_0 = ds[0];
    // n must have at least as many low zero bits as d
    let d_mask = (d_0 & d_0.wrapping_neg()).wrapping_sub(1);
    if ns[0] & d_mask != 0 {
        return false;
    }
    if d_len == 1 {
        return if n_len >= BMOD_1_TO_MOD_1_THRESHOLD {
            limbs_mod_limb(ns, d_0) == 0
        } else {
            _limbs_mod_exact_odd_limb(ns, d_0 >> d_0.trailing_zeros(), 0) == 0
        };
    }
    let trailing_zeros = TrailingZeros::trailing_zeros(d_0);
    if d_len == 2 {
        let d_1 = ds[1];
        if d_1 <= d_mask {
            let d_low = (d_0 >> trailing_zeros) | (d_1 << (Limb::WIDTH - trailing_zeros));
            return if n_len < BMOD_1_TO_MOD_1_THRESHOLD {
                _limbs_mod_exact_odd_limb(ns, d_low, 0)
            } else {
                limbs_mod_limb(ns, d_low)
            } == 0;
        }
    }
    let n_len_plus_1 = n_len + 1;
    let mut rs_qs = vec![0; (n_len_plus_1 << 1) - d_len];
    let (rs, qs) = rs_qs.split_at_mut(n_len_plus_1);
    if trailing_zeros != 0 {
        scratch = vec![0; d_len];
        assert_eq!(limbs_shr_to_out(&mut scratch, ds, trailing_zeros), 0);
        ds = &scratch;
        assert_eq!(limbs_shr_to_out(rs, ns, trailing_zeros), 0);
    } else {
        rs[..n_len].copy_from_slice(ns);
    }
    let r_len = if rs[n_len - 1] >= ds[d_len - 1] {
        n_len_plus_1
    } else if n_len == d_len {
        return false;
    } else {
        n_len
    };
    let rs = &mut rs[..r_len];
    let q_len = r_len - d_len;
    let rs = if d_len < DC_BDIV_QR_THRESHOLD || q_len < DC_BDIV_QR_THRESHOLD {
        let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_mod_schoolbook(qs, rs, ds, d_inv);
        &mut rs[q_len..]
    } else if d_len < MU_BDIV_QR_THRESHOLD {
        let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_mod_divide_and_conquer(qs, rs, ds, d_inv);
        &mut rs[q_len..]
    } else {
        let mut scratch = vec![0; _limbs_modular_div_mod_barrett_scratch_len(r_len, d_len)];
        let ns = rs.to_vec();
        _limbs_modular_div_mod_barrett(qs, rs, &ns, ds, &mut scratch);
        &mut rs[..d_len]
    };
    slice_test_zero(rs)
}

impl Natural {
    fn divisible_by_limb(&self, other: Limb) -> bool {
        match (self, other) {
            (&natural_zero!(), _) => true,
            (_, 0) => false,
            (&Natural(Small(small)), y) => small.divisible_by(y),
            (&Natural(Large(ref limbs)), y) => limbs_divisible_by_limb(limbs, y),
        }
    }

    // Tests whether other is divisible by self
    fn limb_divisible_by_natural(&self, other: Limb) -> bool {
        match (other, self) {
            (0, _) => true,
            (_, natural_zero!()) | (_, &Natural(Large(_))) => false,
            (x, &Natural(Small(small))) => x.divisible_by(small),
        }
    }
}

impl DivisibleBy<Natural> for Natural {
    /// Returns whether a `Natural` is divisible by another `Natural`; in other words, whether the
    /// first `Natural` is a multiple of the second. This means that zero is divisible by any
    /// number, including zero; but a nonzero number is never divisible by zero. Both `Natural`s are
    /// taken by value.
    ///
    /// This function is more efficient than finding a remainder and checking whether it's zero.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::ZERO.divisible_by(Natural::ZERO), true);
    /// assert_eq!(Natural::from(100u32).divisible_by(Natural::from(3u32)), false);
    /// assert_eq!(Natural::from(102u32).divisible_by(Natural::from(3u32)), true);
    /// assert_eq!(Natural::from_str("1000000000000000000000000").unwrap()
    ///     .divisible_by(Natural::from_str("1000000000000").unwrap()), true);
    /// ```
    fn divisible_by(mut self, mut other: Natural) -> bool {
        match (&mut self, &mut other) {
            (x, &mut Natural(Small(y))) => x.divisible_by_limb(y),
            (&mut Natural(Small(x)), y) => y.limb_divisible_by_natural(x),
            (Natural(Large(ref mut xs)), Natural(Large(ref mut ys))) => {
                xs.len() >= ys.len() && limbs_divisible_by(xs, ys)
            }
        }
    }
}

impl<'a> DivisibleBy<&'a Natural> for Natural {
    /// Returns whether a `Natural` is divisible by another `Natural`; in other words, whether the
    /// first `Natural` is a multiple of the second. This means that zero is divisible by any
    /// number, including zero; but a nonzero number is never divisible by zero. The first `Natural`
    /// is taken by value and the second by reference.
    ///
    /// This function is more efficient than finding a remainder and checking whether it's zero.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::ZERO.divisible_by(&Natural::ZERO), true);
    /// assert_eq!(Natural::from(100u32).divisible_by(&Natural::from(3u32)), false);
    /// assert_eq!(Natural::from(102u32).divisible_by(&Natural::from(3u32)), true);
    /// assert_eq!(Natural::from_str("1000000000000000000000000").unwrap()
    ///     .divisible_by(&Natural::from_str("1000000000000").unwrap()), true);
    /// ```
    fn divisible_by(mut self, other: &'a Natural) -> bool {
        match (&mut self, other) {
            (x, &Natural(Small(y))) => x.divisible_by_limb(y),
            (&mut Natural(Small(x)), y) => y.limb_divisible_by_natural(x),
            (Natural(Large(ref mut xs)), &Natural(Large(ref ys))) => {
                xs.len() >= ys.len() && limbs_divisible_by_val_ref(xs, ys)
            }
        }
    }
}

impl<'a> DivisibleBy<Natural> for &'a Natural {
    /// Returns whether a `Natural` is divisible by another `Natural`; in other words, whether the
    /// first `Natural` is a multiple of the second. This means that zero is divisible by any
    /// number, including zero; but a nonzero number is never divisible by zero. The first `Natural`
    /// is taken by reference and the second by value.
    ///
    /// This function is more efficient than finding a remainder and checking whether it's zero.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!((&Natural::ZERO).divisible_by(Natural::ZERO), true);
    /// assert_eq!((&Natural::from(100u32)).divisible_by(Natural::from(3u32)), false);
    /// assert_eq!((&Natural::from(102u32)).divisible_by(Natural::from(3u32)), true);
    /// assert_eq!((&Natural::from_str("1000000000000000000000000").unwrap())
    ///     .divisible_by(Natural::from_str("1000000000000").unwrap()), true);
    /// ```
    fn divisible_by(self, mut other: Natural) -> bool {
        match (self, &mut other) {
            (x, &mut Natural(Small(y))) => x.divisible_by_limb(y),
            (&Natural(Small(x)), y) => y.limb_divisible_by_natural(x),
            (&Natural(Large(ref xs)), Natural(Large(ref mut ys))) => {
                xs.len() >= ys.len() && limbs_divisible_by_ref_val(xs, ys)
            }
        }
    }
}

impl<'a, 'b> DivisibleBy<&'b Natural> for &'a Natural {
    /// Returns whether a `Natural` is divisible by another `Natural`; in other words, whether the
    /// first `Natural` is a multiple of the second. This means that zero is divisible by any
    /// number, including zero; but a nonzero number is never divisible by zero. Both `Natural`s are
    /// taken by reference.
    ///
    /// This function is more efficient than finding a remainder and checking whether it's zero.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!((&Natural::ZERO).divisible_by(&Natural::ZERO), true);
    /// assert_eq!((&Natural::from(100u32)).divisible_by(&Natural::from(3u32)), false);
    /// assert_eq!((&Natural::from(102u32)).divisible_by(&Natural::from(3u32)), true);
    /// assert_eq!((&Natural::from_str("1000000000000000000000000").unwrap())
    ///     .divisible_by(&Natural::from_str("1000000000000").unwrap()), true);
    /// ```
    fn divisible_by(self, other: &'b Natural) -> bool {
        match (self, other) {
            (x, &Natural(Small(y))) => x.divisible_by_limb(y),
            (&Natural(Small(x)), y) => y.limb_divisible_by_natural(x),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                xs.len() >= ys.len() && limbs_divisible_by_ref_ref(xs, ys)
            }
        }
    }
}

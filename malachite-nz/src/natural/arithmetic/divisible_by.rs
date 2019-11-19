use malachite_base::limbs::{limbs_leading_zero_limbs, limbs_test_zero};
use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::basic::integers::PrimitiveInteger;

use natural::arithmetic::div_exact::{
    _limbs_modular_div_mod_barrett, _limbs_modular_div_mod_barrett_scratch_len,
    _limbs_modular_div_mod_divide_and_conquer, _limbs_modular_div_mod_schoolbook,
};
use natural::arithmetic::div_exact_limb::limbs_modular_invert_limb;
use natural::arithmetic::divisible_by_limb::BMOD_1_TO_MOD_1_THRESHOLD;
use natural::arithmetic::eq_limb_mod_limb::limbs_mod_exact_odd_limb;
use natural::arithmetic::mod_limb::limbs_mod_limb;
use natural::arithmetic::shr_u::{limbs_shr_to_out, limbs_slice_shr_in_place};
use natural::Natural::{self, Large, Small};
use platform::{Limb, DC_BDIV_QR_THRESHOLD, MU_BDIV_QR_THRESHOLD};

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
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::divisible_by::limbs_divisible_by;
///
/// assert!(!limbs_divisible_by(&mut [1, 2, 3], &mut [4, 5]));
/// assert!(
///     limbs_divisible_by(&mut [10_200, 20_402, 30_605, 20_402, 10_200], &mut [100, 101, 102])
/// );
/// ```
///
/// This is mpn_divisible_p from mpn/generic/divis.c, where an >= dn and neither are zero.
pub fn limbs_divisible_by(ns: &mut [Limb], ds: &mut [Limb]) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert_ne!(*ns.last().unwrap(), 0);
    assert_ne!(*ds.last().unwrap(), 0);
    // Strip low zero limbs from ds, requiring n == 0 on those.
    let offset = limbs_leading_zero_limbs(ds);
    let (ns_lo, ns) = ns.split_at_mut(offset);
    if !limbs_test_zero(ns_lo) {
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
            limbs_mod_exact_odd_limb(ns, d_0 >> d_0.trailing_zeros(), 0) == 0
        };
    }
    let trailing_zeros = d_0.trailing_zeros();
    if d_len == 2 {
        let d_1 = ds[1];
        if d_1 <= d_mask {
            let d_low = (d_0 >> trailing_zeros) | (d_1 << (Limb::WIDTH - trailing_zeros));
            return if n_len < BMOD_1_TO_MOD_1_THRESHOLD {
                limbs_mod_exact_odd_limb(ns, d_low, 0)
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
        let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_mod_schoolbook(&mut qs, rs, ds, inverse);
        &mut rs[q_len..]
    } else if d_len < MU_BDIV_QR_THRESHOLD {
        let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_mod_divide_and_conquer(&mut qs, rs, ds, inverse);
        &mut rs[q_len..]
    } else {
        let mut scratch = vec![0; _limbs_modular_div_mod_barrett_scratch_len(r_len, d_len)];
        let ns = rs.to_vec();
        _limbs_modular_div_mod_barrett(&mut qs, rs, &ns, ds, &mut scratch);
        &mut rs[..d_len]
    };
    limbs_test_zero(rs)
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
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::divisible_by::limbs_divisible_by_val_ref;
///
/// assert!(!limbs_divisible_by_val_ref(&mut [1, 2, 3], &[4, 5]));
/// assert!(
///     limbs_divisible_by_val_ref(&mut [10_200, 20_402, 30_605, 20_402, 10_200], &[100, 101, 102])
/// );
/// ```
///
/// This is mpn_divisible_p from mpn/generic/divis.c, where an >= dn and neither are zero.
pub fn limbs_divisible_by_val_ref(ns: &mut [Limb], ds: &[Limb]) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert_ne!(*ns.last().unwrap(), 0);
    assert_ne!(*ds.last().unwrap(), 0);
    // Strip low zero limbs from ds, requiring n == 0 on those.
    let offset = limbs_leading_zero_limbs(ds);
    let (ns_lo, ns) = ns.split_at_mut(offset);
    if !limbs_test_zero(ns_lo) {
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
            limbs_mod_exact_odd_limb(ns, d_0 >> d_0.trailing_zeros(), 0) == 0
        };
    }
    let trailing_zeros = d_0.trailing_zeros();
    if d_len == 2 {
        let d_1 = ds[1];
        if d_1 <= d_mask {
            let d_low = (d_0 >> trailing_zeros) | (d_1 << (Limb::WIDTH - trailing_zeros));
            return if n_len < BMOD_1_TO_MOD_1_THRESHOLD {
                limbs_mod_exact_odd_limb(ns, d_low, 0)
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
        let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_mod_schoolbook(&mut qs, rs, ds, inverse);
        &mut rs[q_len..]
    } else if d_len < MU_BDIV_QR_THRESHOLD {
        let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_mod_divide_and_conquer(&mut qs, rs, ds, inverse);
        &mut rs[q_len..]
    } else {
        let mut scratch = vec![0; _limbs_modular_div_mod_barrett_scratch_len(r_len, d_len)];
        let ns = rs.to_vec();
        _limbs_modular_div_mod_barrett(&mut qs, rs, &ns, ds, &mut scratch);
        &mut rs[..d_len]
    };
    limbs_test_zero(rs)
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
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::divisible_by::limbs_divisible_by_ref_val;
///
/// assert!(!limbs_divisible_by_ref_val(&[1, 2, 3], &mut [4, 5]));
/// assert!(
///     limbs_divisible_by_ref_val(&[10_200, 20_402, 30_605, 20_402, 10_200], &mut [100, 101, 102])
/// );
/// ```
///
/// This is mpn_divisible_p from mpn/generic/divis.c, where an >= dn and neither are zero.
pub fn limbs_divisible_by_ref_val(ns: &[Limb], ds: &mut [Limb]) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert_ne!(*ns.last().unwrap(), 0);
    assert_ne!(*ds.last().unwrap(), 0);
    // Strip low zero limbs from ds, requiring n == 0 on those.
    let offset = limbs_leading_zero_limbs(ds);
    let (ns_lo, ns) = ns.split_at(offset);
    if !limbs_test_zero(ns_lo) {
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
            limbs_mod_exact_odd_limb(ns, d_0 >> d_0.trailing_zeros(), 0) == 0
        };
    }
    let trailing_zeros = d_0.trailing_zeros();
    if d_len == 2 {
        let d_1 = ds[1];
        if d_1 <= d_mask {
            let d_low = (d_0 >> trailing_zeros) | (d_1 << (Limb::WIDTH - trailing_zeros));
            return if n_len < BMOD_1_TO_MOD_1_THRESHOLD {
                limbs_mod_exact_odd_limb(ns, d_low, 0)
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
        let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_mod_schoolbook(qs, rs, ds, inverse);
        &mut rs[q_len..]
    } else if d_len < MU_BDIV_QR_THRESHOLD {
        let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_mod_divide_and_conquer(qs, rs, ds, inverse);
        &mut rs[q_len..]
    } else {
        let mut scratch = vec![0; _limbs_modular_div_mod_barrett_scratch_len(r_len, d_len)];
        let ns = rs.to_vec();
        _limbs_modular_div_mod_barrett(qs, rs, &ns, ds, &mut scratch);
        &mut rs[..d_len]
    };
    limbs_test_zero(rs)
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
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::divisible_by::limbs_divisible_by_ref_ref;
///
/// assert!(!limbs_divisible_by_ref_ref(&[1, 2, 3], &[4, 5]));
/// assert!(
///     limbs_divisible_by_ref_ref(&[10_200, 20_402, 30_605, 20_402, 10_200], &[100, 101, 102])
/// );
/// ```
///
/// This is mpn_divisible_p from mpn/generic/divis.c, where an >= dn and neither are zero.
pub fn limbs_divisible_by_ref_ref(ns: &[Limb], ds: &[Limb]) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert_ne!(*ns.last().unwrap(), 0);
    assert_ne!(*ds.last().unwrap(), 0);
    // Strip low zero limbs from ds, requiring n == 0 on those.
    let offset = limbs_leading_zero_limbs(ds);
    let (ns_lo, ns) = ns.split_at(offset);
    if !limbs_test_zero(ns_lo) {
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
            limbs_mod_exact_odd_limb(ns, d_0 >> d_0.trailing_zeros(), 0) == 0
        };
    }
    let trailing_zeros = d_0.trailing_zeros();
    if d_len == 2 {
        let d_1 = ds[1];
        if d_1 <= d_mask {
            let d_low = (d_0 >> trailing_zeros) | (d_1 << (Limb::WIDTH - trailing_zeros));
            return if n_len < BMOD_1_TO_MOD_1_THRESHOLD {
                limbs_mod_exact_odd_limb(ns, d_low, 0)
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
        let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_mod_schoolbook(qs, rs, ds, inverse);
        &mut rs[q_len..]
    } else if d_len < MU_BDIV_QR_THRESHOLD {
        let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_mod_divide_and_conquer(qs, rs, ds, inverse);
        &mut rs[q_len..]
    } else {
        let mut scratch = vec![0; _limbs_modular_div_mod_barrett_scratch_len(r_len, d_len)];
        let ns = rs.to_vec();
        _limbs_modular_div_mod_barrett(qs, rs, &ns, ds, &mut scratch);
        &mut rs[..d_len]
    };
    limbs_test_zero(rs)
}

impl DivisibleBy<Natural> for Natural {
    /// Returns whether a `Natural` is divisible by another `Natural`; in other words, whether the
    /// first `Natural` is a multiple of the second. This means that zero is divisible by any
    /// number, including zero; but a nonzero number is never divisible by zero. Both `Natural`s are
    /// taken by value.
    ///
    /// This method is more efficient than finding a remainder and checking whether it's zero.
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
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.divisible_by(Natural::ZERO), true);
    ///     assert_eq!(Natural::from(100u32).divisible_by(Natural::from(3u32)), false);
    ///     assert_eq!(Natural::from(102u32).divisible_by(Natural::from(3u32)), true);
    ///     assert_eq!(Natural::from_str("1000000000000000000000000").unwrap()
    ///         .divisible_by(Natural::from_str("1000000000000").unwrap()), true);
    /// }
    /// ```
    fn divisible_by(mut self, mut other: Natural) -> bool {
        match (&mut self, &mut other) {
            (x, &mut Small(y)) => (&*x).divisible_by(y),
            (&mut Small(x), y) => x.divisible_by(&*y),
            (Large(ref mut xs), Large(ref mut ys)) => {
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
    /// This method is more efficient than finding a remainder and checking whether it's zero.
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
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.divisible_by(&Natural::ZERO), true);
    ///     assert_eq!(Natural::from(100u32).divisible_by(&Natural::from(3u32)), false);
    ///     assert_eq!(Natural::from(102u32).divisible_by(&Natural::from(3u32)), true);
    ///     assert_eq!(Natural::from_str("1000000000000000000000000").unwrap()
    ///         .divisible_by(&Natural::from_str("1000000000000").unwrap()), true);
    /// }
    /// ```
    fn divisible_by(mut self, other: &'a Natural) -> bool {
        match (&mut self, other) {
            (x, &Small(y)) => (&*x).divisible_by(y),
            (&mut Small(x), y) => x.divisible_by(y),
            (Large(ref mut xs), &Large(ref ys)) => {
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
    /// This method is more efficient than finding a remainder and checking whether it's zero.
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
    /// fn main() {
    ///     assert_eq!((&Natural::ZERO).divisible_by(Natural::ZERO), true);
    ///     assert_eq!((&Natural::from(100u32)).divisible_by(Natural::from(3u32)), false);
    ///     assert_eq!((&Natural::from(102u32)).divisible_by(Natural::from(3u32)), true);
    ///     assert_eq!((&Natural::from_str("1000000000000000000000000").unwrap())
    ///         .divisible_by(Natural::from_str("1000000000000").unwrap()), true);
    /// }
    /// ```
    fn divisible_by(self, mut other: Natural) -> bool {
        match (self, &mut other) {
            (x, &mut Small(y)) => x.divisible_by(y),
            (&Small(x), y) => x.divisible_by(&*y),
            (&Large(ref xs), Large(ref mut ys)) => {
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
    /// This method is more efficient than finding a remainder and checking whether it's zero.
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
    /// fn main() {
    ///     assert_eq!((&Natural::ZERO).divisible_by(&Natural::ZERO), true);
    ///     assert_eq!((&Natural::from(100u32)).divisible_by(&Natural::from(3u32)), false);
    ///     assert_eq!((&Natural::from(102u32)).divisible_by(&Natural::from(3u32)), true);
    ///     assert_eq!((&Natural::from_str("1000000000000000000000000").unwrap())
    ///         .divisible_by(&Natural::from_str("1000000000000").unwrap()), true);
    /// }
    /// ```
    fn divisible_by(self, other: &'b Natural) -> bool {
        match (self, other) {
            (x, &Small(y)) => x.divisible_by(y),
            (&Small(x), y) => x.divisible_by(y),
            (&Large(ref xs), &Large(ref ys)) => {
                xs.len() >= ys.len() && limbs_divisible_by_ref_ref(xs, ys)
            }
        }
    }
}

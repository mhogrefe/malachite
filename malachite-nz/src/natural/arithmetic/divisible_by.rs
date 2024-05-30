// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 2000–2002, 2005, 2009, 2014, 2017, 2018 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::div_exact::{
    limbs_modular_div_mod_barrett, limbs_modular_div_mod_barrett_scratch_len,
    limbs_modular_div_mod_divide_and_conquer, limbs_modular_div_mod_schoolbook,
    limbs_modular_invert_limb,
};
use crate::natural::arithmetic::eq_mod::limbs_mod_exact_odd_limb;
use crate::natural::arithmetic::mod_op::limbs_mod_limb;
use crate::natural::arithmetic::shr::{limbs_shr_to_out, limbs_slice_shr_in_place};
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::{
    Limb, BMOD_1_TO_MOD_1_THRESHOLD, DC_BDIV_QR_THRESHOLD, MU_BDIV_QR_THRESHOLD,
};
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{DivisibleBy, DivisibleByPowerOf2, Parity};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_base::slices::{slice_leading_zeros, slice_test_zero};

// Interpreting a slice of `Limb`s as the limbs of a `Natural` in ascending order, determines
// whether that `Natural` is divisible by a given limb.
//
// This function assumes that `ns` has at least two elements and that `d` is nonzero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// This is equivalent to `mpz_divisible_ui_p` from `mpz/divis_ui.c`, GMP 6.2.1, where `a` is
// non-negative and the `ABOVE_THRESHOLD` branch is excluded.
pub_crate_test! {limbs_divisible_by_limb(ns: &[Limb], d: Limb) -> bool {
    assert!(ns.len() > 1);
    if d.even() {
        let twos = TrailingZeros::trailing_zeros(d);
        ns[0].divisible_by_power_of_2(twos) && limbs_mod_exact_odd_limb(ns, d >> twos, 0) == 0
    } else {
        limbs_mod_exact_odd_limb(ns, d, 0) == 0
    }
}}

fn limbs_mod_limb_helper(ns: &[Limb], d_low: Limb) -> Limb {
    if ns.len() < BMOD_1_TO_MOD_1_THRESHOLD {
        limbs_mod_exact_odd_limb(ns, d_low, 0)
    } else {
        limbs_mod_limb(ns, d_low)
    }
}

// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
// `Natural`s, determines whether the first `Natural` is divisible by the second. Both `Natural`s
// are taken by value.
//
// `ns` must be at least as long as `ds`, both slices must be nonempty, and the last limb of both
// must be nonzero.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `ns` is shorter than `ds`, `ds` is empty, or the last limbs of either slice are zero.
//
// This is equivalent to `mpn_divisible_p` from `mpn/generic/divis.c`, GMP 6.2.1, where `an >= dn`
// and neither are zero.
pub_crate_test! {
#[allow(clippy::absurd_extreme_comparisons)]
limbs_divisible_by(ns: &mut [Limb], ds: &mut [Limb]) -> bool {
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
            limbs_mod_exact_odd_limb(ns, d_0 >> d_0.trailing_zeros(), 0) == 0
        };
    }
    let trailing_zeros = TrailingZeros::trailing_zeros(d_0);
    if d_len == 2 {
        let d_1 = ds[1];
        if d_1 <= d_mask {
            let d_low = (d_0 >> trailing_zeros) | (d_1 << (Limb::WIDTH - trailing_zeros));
            return limbs_mod_limb_helper(ns, d_low) == 0;
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
        limbs_modular_div_mod_schoolbook(&mut qs, rs, ds, d_inv);
        &mut rs[q_len..]
    } else if d_len < MU_BDIV_QR_THRESHOLD {
        let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        limbs_modular_div_mod_divide_and_conquer(&mut qs, rs, ds, d_inv);
        &mut rs[q_len..]
    } else {
        let mut scratch = vec![0; limbs_modular_div_mod_barrett_scratch_len(r_len, d_len)];
        let ns = rs.to_vec();
        limbs_modular_div_mod_barrett(&mut qs, rs, &ns, ds, &mut scratch);
        &mut rs[..d_len]
    };
    slice_test_zero(rs)
}}

// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
// `Natural`s, determines whether the first `Natural` is divisible by the second. The first
// `Natural` is taken by value, and the second by reference.
//
// `ns` must be at least as long as `ds`, both slices must be nonempty, and the last limb of both
// must be nonzero.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `ns` is shorter than `ds`, `ds` is empty, or the last limbs of either slice are zero.
//
// This is equivalent to `mpn_divisible_p` from `mpn/generic/divis.c`, GMP 6.2.1, where `an >= dn`
// and neither are zero.
pub_crate_test! {
#[allow(clippy::absurd_extreme_comparisons)]
limbs_divisible_by_val_ref(ns: &mut [Limb], ds: &[Limb]) -> bool {
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
            limbs_mod_exact_odd_limb(ns, d_0 >> d_0.trailing_zeros(), 0) == 0
        };
    }
    let trailing_zeros = TrailingZeros::trailing_zeros(d_0);
    if d_len == 2 {
        let d_1 = ds[1];
        if d_1 <= d_mask {
            let d_low = (d_0 >> trailing_zeros) | (d_1 << (Limb::WIDTH - trailing_zeros));
            return limbs_mod_limb_helper(ns, d_low) == 0;
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
        limbs_modular_div_mod_schoolbook(&mut qs, rs, ds, d_inv);
        &mut rs[q_len..]
    } else if d_len < MU_BDIV_QR_THRESHOLD {
        let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        limbs_modular_div_mod_divide_and_conquer(&mut qs, rs, ds, d_inv);
        &mut rs[q_len..]
    } else {
        let mut scratch = vec![0; limbs_modular_div_mod_barrett_scratch_len(r_len, d_len)];
        let ns = rs.to_vec();
        limbs_modular_div_mod_barrett(&mut qs, rs, &ns, ds, &mut scratch);
        &mut rs[..d_len]
    };
    slice_test_zero(rs)
}}

// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
// `Natural`s, determines whether the first `Natural` is divisible by the second. The first
// `Natural` is taken by reference, and the second by value.
//
// `ns` must be at least as long as `ds`, both slices must be nonempty, and the last limb of both
// must be nonzero.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `ns` is shorter than `ds`, `ds` is empty, or the last limbs of either slice are zero.
//
// This is equivalent to `mpn_divisible_p` from `mpn/generic/divis.c`, GMP 6.2.1, where `an >= dn`
// and neither are zero.
pub_test! {
#[allow(clippy::absurd_extreme_comparisons)]
limbs_divisible_by_ref_val(ns: &[Limb], ds: &mut [Limb]) -> bool {
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
            limbs_mod_exact_odd_limb(ns, d_0 >> d_0.trailing_zeros(), 0) == 0
        };
    }
    let trailing_zeros = TrailingZeros::trailing_zeros(d_0);
    if d_len == 2 {
        let d_1 = ds[1];
        if d_1 <= d_mask {
            let d_low = (d_0 >> trailing_zeros) | (d_1 << (Limb::WIDTH - trailing_zeros));
            return limbs_mod_limb_helper(ns, d_low) == 0;
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
        limbs_modular_div_mod_schoolbook(qs, rs, ds, d_inv);
        &mut rs[q_len..]
    } else if d_len < MU_BDIV_QR_THRESHOLD {
        let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        limbs_modular_div_mod_divide_and_conquer(qs, rs, ds, d_inv);
        &mut rs[q_len..]
    } else {
        let mut scratch = vec![0; limbs_modular_div_mod_barrett_scratch_len(r_len, d_len)];
        let ns = rs.to_vec();
        limbs_modular_div_mod_barrett(qs, rs, &ns, ds, &mut scratch);
        &mut rs[..d_len]
    };
    slice_test_zero(rs)
}}

// Interpreting two slices of `Limb`s, `ns` and `ds`, as the limbs (in ascending order) of two
// `Natural`s, determines whether the first `Natural` is divisible by the second. Both `Natural`s
// are taken by reference.
//
// `ns` must be at least as long as `ds`, both slices must be nonempty, and the last limb of both
// must be nonzero.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// # Panics
// Panics if `ns` is shorter than `ds`, `ds` is empty, or the last limbs of either slice are zero.
//
// This is equivalent to `mpn_divisible_p` from `mpn/generic/divis.c`, GMP 6.2.1, where `an >= dn`
// and neither are zero.
pub_test! {
#[allow(clippy::absurd_extreme_comparisons)]
limbs_divisible_by_ref_ref(ns: &[Limb], ds: &[Limb]) -> bool {
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
            limbs_mod_exact_odd_limb(ns, d_0 >> d_0.trailing_zeros(), 0) == 0
        };
    }
    let trailing_zeros = TrailingZeros::trailing_zeros(d_0);
    if d_len == 2 {
        let d_1 = ds[1];
        if d_1 <= d_mask {
            let d_low = (d_0 >> trailing_zeros) | (d_1 << (Limb::WIDTH - trailing_zeros));
            return limbs_mod_limb_helper(ns, d_low) == 0;
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
        limbs_modular_div_mod_schoolbook(qs, rs, ds, d_inv);
        &mut rs[q_len..]
    } else if d_len < MU_BDIV_QR_THRESHOLD {
        let d_inv = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        limbs_modular_div_mod_divide_and_conquer(qs, rs, ds, d_inv);
        &mut rs[q_len..]
    } else {
        let mut scratch = vec![0; limbs_modular_div_mod_barrett_scratch_len(r_len, d_len)];
        let ns = rs.to_vec();
        limbs_modular_div_mod_barrett(qs, rs, &ns, ds, &mut scratch);
        &mut rs[..d_len]
    };
    slice_test_zero(rs)
}}

impl Natural {
    fn divisible_by_limb(&self, other: Limb) -> bool {
        match (self, other) {
            (&Natural::ZERO, _) => true,
            (_, 0) => false,
            (&Natural(Small(small)), y) => small.divisible_by(y),
            (&Natural(Large(ref limbs)), y) => limbs_divisible_by_limb(limbs, y),
        }
    }

    // Tests whether other is divisible by self
    fn limb_divisible_by_natural(&self, other: Limb) -> bool {
        match (other, self) {
            (0, _) => true,
            (_, &Natural::ZERO | &Natural(Large(_))) => false,
            (x, &Natural(Small(small))) => x.divisible_by(small),
        }
    }
}

impl DivisibleBy<Natural> for Natural {
    /// Returns whether a [`Natural`] is divisible by another [`Natural`]; in other words, whether
    /// the first is a multiple of the second. Both [`Natural`]s are taken by value.
    ///
    /// This means that zero is divisible by any [`Natural`], including zero; but a nonzero
    /// [`Natural`] is never divisible by zero.
    ///
    /// It's more efficient to use this function than to compute the remainder and check whether
    /// it's zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.divisible_by(Natural::ZERO), true);
    /// assert_eq!(
    ///     Natural::from(100u32).divisible_by(Natural::from(3u32)),
    ///     false
    /// );
    /// assert_eq!(
    ///     Natural::from(102u32).divisible_by(Natural::from(3u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     Natural::from_str("1000000000000000000000000")
    ///         .unwrap()
    ///         .divisible_by(Natural::from_str("1000000000000").unwrap()),
    ///     true
    /// );
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
    /// Returns whether a [`Natural`] is divisible by another [`Natural`]; in other words, whether
    /// the first is a multiple of the second. The first [`Natural`]s is taken by reference and the
    /// second by value.
    ///
    /// This means that zero is divisible by any [`Natural`], including zero; but a nonzero
    /// [`Natural`] is never divisible by zero.
    ///
    /// It's more efficient to use this function than to compute the remainder and check whether
    /// it's zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.divisible_by(&Natural::ZERO), true);
    /// assert_eq!(
    ///     Natural::from(100u32).divisible_by(&Natural::from(3u32)),
    ///     false
    /// );
    /// assert_eq!(
    ///     Natural::from(102u32).divisible_by(&Natural::from(3u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     Natural::from_str("1000000000000000000000000")
    ///         .unwrap()
    ///         .divisible_by(&Natural::from_str("1000000000000").unwrap()),
    ///     true
    /// );
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
    /// Returns whether a [`Natural`] is divisible by another [`Natural`]; in other words, whether
    /// the first is a multiple of the second. The first [`Natural`]s are taken by reference and the
    /// second by value.
    ///
    /// This means that zero is divisible by any [`Natural`], including zero; but a nonzero
    /// [`Natural`] is never divisible by zero.
    ///
    /// It's more efficient to use this function than to compute the remainder and check whether
    /// it's zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::ZERO).divisible_by(Natural::ZERO), true);
    /// assert_eq!(
    ///     (&Natural::from(100u32)).divisible_by(Natural::from(3u32)),
    ///     false
    /// );
    /// assert_eq!(
    ///     (&Natural::from(102u32)).divisible_by(Natural::from(3u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Natural::from_str("1000000000000000000000000").unwrap())
    ///         .divisible_by(Natural::from_str("1000000000000").unwrap()),
    ///     true
    /// );
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
    /// Returns whether a [`Natural`] is divisible by another [`Natural`]; in other words, whether
    /// the first is a multiple of the second. Both [`Natural`]s are taken by reference.
    ///
    /// This means that zero is divisible by any [`Natural`], including zero; but a nonzero
    /// [`Natural`] is never divisible by zero.
    ///
    /// It's more efficient to use this function than to compute the remainder and check whether
    /// it's zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::ZERO).divisible_by(&Natural::ZERO), true);
    /// assert_eq!(
    ///     (&Natural::from(100u32)).divisible_by(&Natural::from(3u32)),
    ///     false
    /// );
    /// assert_eq!(
    ///     (&Natural::from(102u32)).divisible_by(&Natural::from(3u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Natural::from_str("1000000000000000000000000").unwrap())
    ///         .divisible_by(&Natural::from_str("1000000000000").unwrap()),
    ///     true
    /// );
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

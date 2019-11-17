use malachite_base::num::basic::integers::PrimitiveInteger;

use natural::arithmetic::div_exact::{
    _limbs_modular_div_mod_barrett, _limbs_modular_div_mod_barrett_scratch_len,
    _limbs_modular_div_mod_divide_and_conquer, _limbs_modular_div_mod_schoolbook,
};
use natural::arithmetic::div_exact_limb::limbs_modular_invert_limb;
use natural::arithmetic::divisible_by_limb::BMOD_1_TO_MOD_1_THRESHOLD;
use natural::arithmetic::eq_limb_mod_limb::limbs_mod_exact_odd_limb;
use natural::arithmetic::mod_limb::limbs_mod_limb;
use natural::arithmetic::shr_u::limbs_shr_to_out;
use platform::{Limb, DC_BDIV_QR_THRESHOLD, MU_BDIV_QR_THRESHOLD};

/// This is mpn_divisible_p from mpn/generic/divis.c, where an >= dn and neither are zero.
pub fn limbs_divisible_by(ns: &[Limb], ds: &[Limb]) -> bool {
    let mut n_len = ns.len();
    let mut d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert_ne!(*ns.last().unwrap(), 0);
    assert_ne!(*ds.last().unwrap(), 0);
    // Strip low zero limbs from d, requiring a==0 on those.
    let mut offset = 0;
    let mut n_low;
    let mut d_low;
    loop {
        n_low = ns[offset];
        d_low = ds[offset];
        if d_low != 0 {
            break;
        }
        if n_low != 0 {
            // a has fewer low zero limbs than d, so not divisible
            return false;
        }
        // a != 0 and d != 0, so won't get to n == 0
        n_len -= 1;
        assert!(n_len > 0);
        d_len -= 1;
        assert!(d_len > 0);
        offset += 1;
    }
    let ns = &ns[offset..];
    let mut scratch;
    let mut ds = &ds[offset..];
    // a must have at least as many low zero bits as d
    let d_mask = (d_low & d_low.wrapping_neg()).wrapping_sub(1);
    if n_low & d_mask != 0 {
        return false;
    }
    if d_len == 1 {
        if n_len >= BMOD_1_TO_MOD_1_THRESHOLD {
            return limbs_mod_limb(&ns[..n_len], d_low) == 0;
        }
        let twos = d_low.trailing_zeros();
        d_low >>= twos;
        return limbs_mod_exact_odd_limb(&ns[..n_len], d_low, 0) == 0;
    }
    let twos = d_low.trailing_zeros();
    if d_len == 2 {
        let d_second = ds[1];
        if d_second <= d_mask {
            d_low = (d_low >> twos) | (d_second << (Limb::WIDTH - twos));
            return if n_len < BMOD_1_TO_MOD_1_THRESHOLD {
                limbs_mod_exact_odd_limb(&ns[..n_len], d_low, 0)
            } else {
                limbs_mod_limb(&ns[..n_len], d_low)
            } == 0;
        }
    }
    let mut rs_qs = vec![0; 2 * n_len - d_len + 2];
    let (rs, qs) = rs_qs.split_at_mut(n_len + 1);
    if twos != 0 {
        scratch = vec![0; d_len];
        assert_eq!(limbs_shr_to_out(&mut scratch, &ds[..d_len], twos), 0);
        ds = &scratch;
        assert_eq!(limbs_shr_to_out(rs, &ns[..n_len], twos), 0);
    } else {
        rs[..n_len].copy_from_slice(&ns[..n_len]);
    }
    if rs[n_len - 1] >= ds[d_len - 1] {
        rs[n_len] = 0;
        n_len += 1;
    } else if n_len == d_len {
        return false;
    }
    assert!(n_len > d_len); // requirement of functions below
    let rs = if d_len < DC_BDIV_QR_THRESHOLD || n_len - d_len < DC_BDIV_QR_THRESHOLD {
        let di = limbs_modular_invert_limb(ds[0]);
        _limbs_modular_div_mod_schoolbook(qs, &mut rs[..n_len], &ds[..d_len], di.wrapping_neg());
        &mut rs[n_len - d_len..]
    } else if d_len < MU_BDIV_QR_THRESHOLD {
        let di = limbs_modular_invert_limb(ds[0]);
        _limbs_modular_div_mod_divide_and_conquer(
            qs,
            &mut rs[..n_len],
            &ds[..d_len],
            di.wrapping_neg(),
        );
        &mut rs[n_len - d_len..]
    } else {
        let mut scratch = vec![0; _limbs_modular_div_mod_barrett_scratch_len(n_len, d_len)];
        let ns = rs[..n_len].to_vec();
        _limbs_modular_div_mod_barrett(qs, rs, &ns, &ds[..d_len], &mut scratch);
        rs
    };
    // test for &rp[..dn] zero or non-zero
    let mut i = 0;
    loop {
        if rs[i] != 0 {
            return false;
        }
        i += 1;
        if i >= d_len {
            break;
        }
    }
    true
}

use malachite_base::num::arithmetic::traits::{Parity, WrappingSubAssign};

use natural::arithmetic::add::{
    limbs_slice_add_greater_in_place_left, limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::add_limb::limbs_slice_add_limb_in_place;
use natural::arithmetic::add_mul_limb::limbs_slice_add_mul_limb_same_length_in_place_left;
use natural::arithmetic::mul::mul_low::limbs_mul_low_same_length;
use natural::arithmetic::mul::{limbs_mul_greater_to_out, limbs_mul_to_out};
use natural::arithmetic::sub::{limbs_sub_in_place_left, limbs_sub_same_length_in_place_left};
use natural::arithmetic::sub_limb::limbs_sub_limb_in_place;
use natural::arithmetic::sub_mul_limb::limbs_sub_mul_limb_same_length_in_place_left;
use platform::Limb;

/// Computes a binary quotient of size `q_len` = `ns.len()` - `ds.len()`. D must be odd. `inverse`
/// is (-D) ^ -1 mod 2 ^ `Limb::WIDTH`, or `limbs_modular_invert_limb(ds[0]).wrapping_neg()`.
///
/// Output:
///    Q = N / D mod 2 ^ (`Limb::WIDTH` * `q_len`)
///    R = (N - Q * D) / 2 ^ (`Limb::WIDTH` * `q_len`)
///
/// Stores the `ds.len()` least-significant limbs of R at `&np[q_len..]` and returns the borrow from
/// the subtraction N - Q * D.
///
/// Time: worst case O(n ^ 2)
///
/// where n = `ns.len()`
///
/// Additional memory: worst case O(1)
///
/// This is mpn_sbpi1_bdiv_qr from mpn/generic/sbpi1_bdiv_qr.c.
pub fn _limbs_modular_div_mod_schoolbook(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    inverse: Limb,
) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len > d_len);
    assert!(ds[0].odd());
    let q_len = n_len - d_len;
    let qs = &mut qs[..q_len];
    let mut highest_r = false;
    // To complete the negation, this value is added to the quotient.
    let mut lowest_q = true;
    let mut q_len_s = q_len;
    while q_len_s > d_len {
        let q_diff = q_len - q_len_s;
        for i in q_diff..n_len - q_len_s {
            let ns = &mut ns[i..i + d_len];
            let q = inverse.wrapping_mul(ns[0]);
            ns[0] = limbs_slice_add_mul_limb_same_length_in_place_left(ns, ds, q);
            qs[i] = !q;
        }
        let (np_lo, np_hi) = ns[q_diff..].split_at_mut(d_len);
        if limbs_slice_add_greater_in_place_left(&mut np_hi[..q_len_s], np_lo) {
            highest_r = true;
        }
        if lowest_q && !limbs_slice_add_limb_in_place(&mut qs[q_diff..n_len - q_len_s], 1) {
            lowest_q = false;
        }
        q_len_s -= d_len;
    }
    let q_len_s = q_len_s;
    let q_diff = q_len - q_len_s;
    for i in q_diff..q_len {
        let ns = &mut ns[i..i + d_len];
        let q = inverse.wrapping_mul(ns[0]);
        ns[0] = limbs_slice_add_mul_limb_same_length_in_place_left(ns, ds, q);
        qs[i] = !q;
    }
    let (np_lo, np_hi) = ns[q_diff..].split_at_mut(d_len);
    if limbs_slice_add_same_length_in_place_left(&mut np_hi[..q_len_s], &np_lo[..q_len_s]) {
        assert!(!highest_r);
        highest_r = true;
    }
    if lowest_q && limbs_slice_add_limb_in_place(&mut qs[q_diff..], 1) {
        // quotient is zero
        assert!(!highest_r);
        false
    } else {
        let carry = limbs_sub_same_length_in_place_left(&mut ns[q_len..], ds);
        assert!(carry || !highest_r);
        carry != highest_r
    }
}

//TODO tune
const DC_BDIV_QR_THRESHOLD: usize = 44;

/// Time: worst case O(n * log(n) ^ 2 * log(log(n)))
///
/// Additional memory: worst case O(n * log(n))
///
/// where n = `ds.len()`
///
/// This is mpn_dcpi1_bdiv_qr_n from mpn/generic/dcpi1_bdiv_qr.c.
fn _limbs_modular_div_mod_divide_and_conquer_helper(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    inverse: Limb,
    scratch: &mut [Limb],
) -> bool {
    let n = ds.len();
    let ns = &mut ns[..n << 1];
    let scratch = &mut scratch[..n];
    let lo = n >> 1; // floor(n / 2)
    let hi = n - lo; // ceil(n / 2)
    let (ds_lo, ds_hi) = ds.split_at(lo);
    let carry = if lo < DC_BDIV_QR_THRESHOLD {
        _limbs_modular_div_mod_schoolbook(qs, &mut ns[..lo << 1], ds_lo, inverse)
    } else {
        _limbs_modular_div_mod_divide_and_conquer_helper(qs, ns, ds_lo, inverse, scratch)
    };
    let (qs_lo, qs_hi) = qs.split_at_mut(lo);
    limbs_mul_greater_to_out(scratch, ds_hi, qs_lo);
    if carry {
        assert!(!limbs_slice_add_limb_in_place(&mut scratch[lo..], 1));
    }
    let ns = &mut ns[lo..];
    let highest_r = limbs_sub_in_place_left(ns, scratch);
    let (ds_lo, ds_hi) = ds.split_at(hi);
    let carry = if hi < DC_BDIV_QR_THRESHOLD {
        _limbs_modular_div_mod_schoolbook(qs_hi, &mut ns[..hi << 1], ds_lo, inverse)
    } else {
        _limbs_modular_div_mod_divide_and_conquer_helper(qs_hi, ns, ds_lo, inverse, scratch)
    };
    limbs_mul_greater_to_out(scratch, &qs_hi[..hi], ds_hi);
    if carry {
        assert!(!limbs_slice_add_limb_in_place(&mut scratch[hi..], 1));
    }
    if limbs_sub_same_length_in_place_left(&mut ns[hi..], scratch) {
        assert!(!highest_r);
        true
    } else {
        highest_r
    }
}

/// Computes a binary quotient of size `q_len` = `ns.len()` - `ds.len()`. D must be odd. `inverse`
/// is (-D) ^ -1 mod 2 ^ `Limb::WIDTH`, or `limbs_modular_invert_limb(ds[0]).wrapping_neg()`.
///
/// Output:
///    Q = N / D mod 2 ^ (`Limb::WIDTH` * `q_len`)
///    R = (N - Q * D) / 2 ^ (`Limb::WIDTH` * `q_len`)
///
/// Stores the `ds.len()` least-significant limbs of R at `&np[q_len..]` and returns the borrow from
/// the subtraction N - Q * D.
///
/// Time: worst case O(n * log(d) ^ 2 * log(log(d)))
///
/// Additional memory: worst case O(n * log(n))
///
/// where n = `ns.len()`, d = `ds.len()`
///
/// This is mpn_dcpi1_bdiv_qr from mpn/generic/dcpi1_bdiv_qr.c.
pub fn _limbs_modular_div_mod_divide_and_conquer(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    inverse: Limb,
) -> bool {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(d_len >= 2); // to adhere to _limbs_modular_div_mod_schoolbook's limits
    assert!(n_len > d_len); // to adhere to _limbs_modular_div_mod_schoolbook's limits
    assert!(ds[0].odd());
    let mut scratch = vec![0; d_len];
    let q_len = n_len - d_len;
    let qs = &mut qs[..q_len];
    let mut borrow = false;
    let mut carry;
    if q_len > d_len {
        let q_len_mod_d_len = {
            let mut m = q_len % d_len;
            if m == 0 {
                m = d_len;
            }
            m
        };
        let (ds_lo, ds_hi) = ds.split_at(q_len_mod_d_len);
        // Perform the typically smaller block first.
        carry = if q_len_mod_d_len < DC_BDIV_QR_THRESHOLD {
            _limbs_modular_div_mod_schoolbook(qs, &mut ns[..q_len_mod_d_len << 1], ds_lo, inverse)
        } else {
            _limbs_modular_div_mod_divide_and_conquer_helper(qs, ns, ds_lo, inverse, &mut scratch)
        };
        if q_len_mod_d_len != d_len {
            limbs_mul_to_out(&mut scratch, ds_hi, &qs[..q_len_mod_d_len]);
            if carry {
                assert!(!limbs_slice_add_limb_in_place(
                    &mut scratch[q_len_mod_d_len..],
                    1
                ));
            }
            borrow = limbs_sub_in_place_left(&mut ns[q_len_mod_d_len..], &scratch[..d_len]);
            carry = false;
        }
        let mut q_len_s = q_len - q_len_mod_d_len; // q_len_s is a multiple of d_len
        while q_len_s != 0 {
            let q_diff = q_len - q_len_s;
            let ns = &mut ns[q_diff..];
            if carry && limbs_sub_limb_in_place(&mut ns[d_len..], 1) {
                assert!(!borrow);
                borrow = true;
            }
            carry = _limbs_modular_div_mod_divide_and_conquer_helper(
                &mut qs[q_diff..],
                ns,
                ds,
                inverse,
                &mut scratch,
            );
            q_len_s -= d_len;
        }
    } else {
        let (ds_lo, ds_hi) = ds.split_at(q_len);
        carry = if q_len < DC_BDIV_QR_THRESHOLD {
            _limbs_modular_div_mod_schoolbook(qs, &mut ns[..q_len << 1], ds_lo, inverse)
        } else {
            _limbs_modular_div_mod_divide_and_conquer_helper(qs, ns, ds_lo, inverse, &mut scratch)
        };
        if q_len != d_len {
            limbs_mul_to_out(&mut scratch, ds_hi, qs);
            if carry {
                assert!(!limbs_slice_add_limb_in_place(&mut scratch[q_len..], 1));
            }
            borrow = limbs_sub_in_place_left(&mut ns[q_len..], &scratch[..d_len]);
            carry = false;
        }
    }
    if carry {
        assert!(!borrow);
        borrow = true;
    }
    borrow
}

/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// This is mpn_dcpi1_bdiv_qr_n_itch from mpn/generic/dcpi1_bdiv_qr.c.
#[inline]
pub const fn _limbs_modular_div_mod_divide_and_conquer_helper_scratch_len(n: usize) -> usize {
    n
}

/// Computes Q = N / D mod 2 ^ (`Limb::WIDTH` * `ns.len()`), destroying N. D must be odd. `inverse`
/// is (-D) ^ -1 mod 2 ^ `Limb::WIDTH`, or `limbs_modular_invert_limb(ds[0]).wrapping_neg()`.
///
/// The straightforward way to compute Q is to cancel one limb at a time, using
///     qs[i] = D ^ (-1) * ns[i] mod 2 ^ `Limb::WIDTH`
///     N -= 2 ^ (Limb::WIDTH * i) * qs[i] * D
///
/// But we prefer addition to subtraction, since
/// `limbs_slice_add_mul_limb_same_length_in_place_left` is often faster than
/// `limbs_sub_mul_limb_same_length_in_place_left`. Q = -N / D can be computed by iterating
///     qs[i] = (-D) ^ (-1) * ns[i] mod 2 ^ `Limb::WIDTH`
///     N += 2 ^ (Limb::WIDTH * i) * qs[i] * D
///
/// And then we flip the sign: -Q = ~Q + 1.
///
/// Time: worst case O(n ^ 2)
///
/// Additional memory: worst case O(1)
///
/// where n = `ns.len()`
///
/// This is mpn_sbpi1_bdiv_q from mpn/generic/sbpi1_bdiv_q.c.
pub fn _limbs_modular_div_schoolbook(qs: &mut [Limb], ns: &mut [Limb], ds: &[Limb], inverse: Limb) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert_ne!(d_len, 0);
    assert!(n_len >= d_len);
    assert!(ds[0].odd());
    let qs = &mut qs[..n_len];
    let diff = n_len - d_len;
    for i in 0..diff {
        let q = inverse.wrapping_mul(ns[i]);
        let (ns_lo, ns_hi) = ns[i..].split_at_mut(d_len);
        let carry = limbs_slice_add_mul_limb_same_length_in_place_left(ns_lo, ds, q);
        limbs_slice_add_limb_in_place(ns_hi, carry);
        assert_eq!(ns_lo[0], 0);
        qs[i] = !q;
    }
    let last_index = n_len - 1;
    for i in diff..last_index {
        let ns_hi = &mut ns[i..];
        let q = inverse.wrapping_mul(ns_hi[0]);
        limbs_slice_add_mul_limb_same_length_in_place_left(ns_hi, &ds[..n_len - i], q);
        assert_eq!(ns_hi[0], 0);
        qs[i] = !q;
    }
    qs[last_index] = !inverse.wrapping_mul(ns[last_index]);
    limbs_slice_add_limb_in_place(qs, 1);
}

//TODO tune
const DC_BDIV_Q_THRESHOLD: usize = 104;

/// Time: worst case O(n * log(n) ^ 2 * log(log(n)))
///
/// Additional memory: worst case O(n * log(n))
///
/// where n = `ds.len()`
///
/// This is mpn_dcpi1_bdiv_q_n from mpn/generic/dcpi1_bdiv_q.c.
fn _limbs_modular_div_divide_and_conquer_helper(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    inverse: Limb,
    scratch: &mut [Limb],
) {
    let n = ds.len();
    let mut n_rem = n;
    while n_rem >= DC_BDIV_Q_THRESHOLD {
        let m = n - n_rem;
        let lo = n_rem >> 1; // floor(n / 2)
        let hi = n_rem - lo; // ceil(n / 2)
        let qs = &mut qs[m..];
        let ns = &mut ns[m..];
        let carry_1 =
            _limbs_modular_div_mod_divide_and_conquer_helper(qs, ns, &ds[..lo], inverse, scratch);
        let qs = &qs[..lo];
        limbs_mul_low_same_length(scratch, qs, &ds[hi..n_rem]);
        limbs_sub_same_length_in_place_left(&mut ns[hi..n_rem], &scratch[..lo]);
        if lo < hi {
            let carry_2 =
                limbs_sub_mul_limb_same_length_in_place_left(&mut ns[lo..lo << 1], qs, ds[lo]);
            let n_limb = &mut ns[n_rem - 1];
            n_limb.wrapping_sub_assign(carry_2);
            if carry_1 {
                n_limb.wrapping_sub_assign(1);
            }
        }
        n_rem = hi;
    }
    let m = n - n_rem;
    _limbs_modular_div_schoolbook(&mut qs[m..], &mut ns[m..n], &ds[..n_rem], inverse);
}

/// Computes Q = N / D mod 2 ^ (`Limb::WIDTH` * `ns.len()`), destroying N. D must be odd. `inverse`
/// is (-D) ^ -1 mod 2 ^ `Limb::WIDTH`, or `limbs_modular_invert_limb(ds[0]).wrapping_neg()`.
///
/// Time: worst case O(n * log(d) ^ 2 * log(log(d)))
///
/// Additional memory: worst case O(n * log(n))
///
/// where n = `ns.len()`, d = `ds.len()`
///
/// This is mpn_dcpi1_bdiv_q from mpn/generic/dcpi1_bdiv_q.c.
pub fn _limbs_modular_div_divide_and_conquer(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    inverse: Limb,
) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(d_len >= 2);
    assert!(n_len >= d_len);
    assert!(ds[0].odd());
    if n_len > d_len {
        let n_len_mod_d_len = {
            let mut m = n_len % d_len;
            if m == 0 {
                m = d_len;
            }
            m
        };
        let mut scratch = vec![0; d_len];
        // Perform the typically smaller block first.
        let (ds_lo, ds_hi) = ds.split_at(n_len_mod_d_len);
        let mut carry = if n_len_mod_d_len < DC_BDIV_QR_THRESHOLD {
            _limbs_modular_div_mod_schoolbook(qs, &mut ns[..n_len_mod_d_len << 1], ds_lo, inverse)
        } else {
            _limbs_modular_div_mod_divide_and_conquer_helper(qs, ns, ds_lo, inverse, &mut scratch)
        };
        if n_len_mod_d_len != d_len {
            limbs_mul_to_out(&mut scratch, ds_hi, &qs[..n_len_mod_d_len]);
            if carry {
                assert!(!limbs_slice_add_limb_in_place(
                    &mut scratch[n_len_mod_d_len..],
                    1
                ));
            }
            limbs_sub_in_place_left(&mut ns[n_len_mod_d_len..], &scratch[..d_len]);
            carry = false;
        }
        let mut m = n_len_mod_d_len;
        let diff = n_len - d_len;
        while m != diff {
            if carry {
                limbs_sub_limb_in_place(&mut ns[m + d_len..], 1);
            }
            carry = _limbs_modular_div_mod_divide_and_conquer_helper(
                &mut qs[m..],
                &mut ns[m..],
                ds,
                inverse,
                &mut scratch,
            );
            m += d_len;
        }
        _limbs_modular_div_divide_and_conquer_helper(
            &mut qs[diff..],
            &mut ns[diff..],
            ds,
            inverse,
            &mut scratch,
        );
    } else {
        if n_len < DC_BDIV_Q_THRESHOLD {
            _limbs_modular_div_schoolbook(qs, ns, ds, inverse);
        } else {
            let mut scratch = vec![0; n_len];
            _limbs_modular_div_divide_and_conquer_helper(qs, ns, ds, inverse, &mut scratch);
        }
    }
}

/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// This is mpn_dcpi1_bdiv_q_n_itch from mpn/generic/dcpi1_bdiv_q.c.
pub const fn _limbs_modular_div_divide_and_conquer_helper_scratch_len(n: usize) -> usize {
    n
}

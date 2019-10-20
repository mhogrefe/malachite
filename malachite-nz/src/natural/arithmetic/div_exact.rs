use malachite_base::num::arithmetic::traits::Parity;

use natural::arithmetic::add::{
    limbs_slice_add_greater_in_place_left, limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::add_limb::limbs_slice_add_limb_in_place;
use natural::arithmetic::add_mul_limb::limbs_slice_add_mul_limb_same_length_in_place_left;
use natural::arithmetic::sub::limbs_sub_same_length_in_place_left;
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

use natural::arithmetic::add::{
    _limbs_add_to_out_special, limbs_add_same_length_to_out, limbs_add_to_out,
    limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::shl_u::{limbs_shl_to_out, limbs_slice_shl_in_place};
use natural::arithmetic::sub::limbs_sub_same_length_to_out;
use natural::comparison::ord::limbs_cmp_same_length;
use platform::Limb;
use std::cmp::Ordering;

/// Evaluate a degree-3 polynomial in +1 and -1, where each coefficient has width `n` limbs, except
/// the last, which has width `n_high` limbs.
///
/// This is mpn_toom_eval_dgr3_pm1 in mpn/generic/toom_eval_dgr3_pm1.c.
pub(crate) fn _limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1(
    v_1: &mut [Limb],
    v_neg_1: &mut [Limb],
    poly: &[Limb],
    n: usize,
    n_high: usize,
    scratch: &mut [Limb],
) -> bool {
    assert_ne!(n_high, 0);
    assert!(n_high <= n);
    assert_eq!(v_1.len(), n + 1);
    assert_eq!(scratch.len(), n + 1);

    let mut poly_chunks_iter = poly.chunks_exact(n);
    let poly_0 = poly_chunks_iter.next().unwrap();
    let poly_1 = poly_chunks_iter.next().unwrap();
    let poly_2 = poly_chunks_iter.next().unwrap();
    let poly_3 = if let Some(last_full_size_chunk) = poly_chunks_iter.next() {
        last_full_size_chunk
    } else {
        poly_chunks_iter.remainder()
    };
    assert_eq!(poly_3.len(), n_high);
    v_1[n] = if limbs_add_same_length_to_out(v_1, poly_0, poly_2) {
        1
    } else {
        0
    };
    scratch[n] = if limbs_add_to_out(scratch, poly_1, poly_3) {
        1
    } else {
        0
    };
    let v_neg_1_neg = limbs_cmp_same_length(v_1, scratch) == Ordering::Less;
    if v_neg_1_neg {
        limbs_sub_same_length_to_out(v_neg_1, scratch, v_1);
    } else {
        limbs_sub_same_length_to_out(v_neg_1, v_1, scratch);
    }
    limbs_slice_add_same_length_in_place_left(v_1, scratch);
    assert!(v_1[n] <= 3);
    assert!(v_neg_1[n] <= 1);
    v_neg_1_neg
}

/// Evaluate a degree-3 polynomial in +2 and -2, where each coefficient has width `n` limbs, except
/// the last, which has width `n_high` limbs.
///
/// Needs n + 1 limbs of temporary storage.
/// This is mpn_toom_eval_dgr3_pm2 from mpn/generic/toom_eval_dg3_pm2.c.
pub(crate) fn _limbs_mul_toom_evaluate_deg_3_poly_in_2_and_neg_2(
    v_2: &mut [Limb],
    v_neg_2: &mut [Limb],
    poly: &[Limb],
    n: usize,
    high_n: usize,
    scratch: &mut [Limb],
) -> bool {
    assert_ne!(high_n, 0);
    assert!(high_n <= n);
    assert_eq!(v_2.len(), n + 1);
    {
        let (scratch_last, scratch_init) = scratch.split_last_mut().unwrap();
        assert_eq!(scratch_init.len(), n);

        let mut poly_chunks_iter = poly.chunks_exact(n);
        let poly_0 = poly_chunks_iter.next().unwrap();
        let poly_1 = poly_chunks_iter.next().unwrap();
        let poly_2 = poly_chunks_iter.next().unwrap();
        let poly_3 = if let Some(last_full_size_chunk) = poly_chunks_iter.next() {
            last_full_size_chunk
        } else {
            poly_chunks_iter.remainder()
        };
        assert_eq!(poly_3.len(), high_n);
        // scratch <- (poly_0 + 4 * poly_2) +/- (2 * poly_1 + 8 * poly_3)
        v_2[n] = limbs_shl_to_out(scratch_init, poly_2, 2);
        if limbs_add_same_length_to_out(v_2, scratch_init, poly_0) {
            v_2[n] += 1;
        }
        if high_n < n {
            scratch_init[high_n] = limbs_shl_to_out(scratch_init, poly_3, 2);
            *scratch_last = if _limbs_add_to_out_special(scratch_init, high_n + 1, poly_1) {
                1
            } else {
                0
            };
        } else {
            *scratch_last = limbs_shl_to_out(scratch_init, poly_3, 2);
            if limbs_slice_add_same_length_in_place_left(scratch_init, poly_1) {
                *scratch_last += 1;
            }
        }
    }
    limbs_slice_shl_in_place(scratch, 1);
    let v_neg_2_neg = limbs_cmp_same_length(v_2, scratch) == Ordering::Less;
    if v_neg_2_neg {
        limbs_sub_same_length_to_out(v_neg_2, scratch, v_2);
    } else {
        limbs_sub_same_length_to_out(v_neg_2, v_2, scratch);
    }
    limbs_slice_add_same_length_in_place_left(v_2, scratch);
    assert!(v_2[n] < 15);
    assert!(v_neg_2[n] < 10);
    v_neg_2_neg
}

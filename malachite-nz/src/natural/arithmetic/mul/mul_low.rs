use malachite_base::num::arithmetic::traits::WrappingAddAssign;

use natural::arithmetic::add::{
    limbs_add_same_length_to_out, limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::add_mul_limb::limbs_slice_add_mul_limb_same_length_in_place_left;
use natural::arithmetic::mul::toom::{TUNE_PROGRAM_BUILD, WANT_FAT_BINARY};
use natural::arithmetic::mul::{_limbs_mul_greater_to_out_basecase, limbs_mul_same_length_to_out};
use natural::arithmetic::mul_limb::limbs_mul_limb_to_out;
use platform::Limb;
use platform::{
    MUL_TOOM22_THRESHOLD, MUL_TOOM33_THRESHOLD, MUL_TOOM44_THRESHOLD, MUL_TOOM8H_THRESHOLD,
};

/// Time: worst case O(n<sup>2</sup>)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// This is mpn_mullo_basecase from mpn/generic/mullo_basecase.c, MULLO_VARIANT == 2
pub fn _limbs_mul_low_same_length_basecase(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let n = xs.len();
    assert_ne!(n, 0);
    assert_eq!(ys.len(), n);
    let (ys_last, ys_init) = ys.split_last().unwrap();
    let (out_last, out_init) = out[..n].split_last_mut().unwrap();
    let mut p = xs[0].wrapping_mul(*ys_last);
    if n != 1 {
        let y = ys_init[0];
        let (xs_last, xs_init) = xs.split_last().unwrap();
        let limb_p = xs_last
            .wrapping_mul(y)
            .wrapping_add(limbs_mul_limb_to_out(out_init, xs_init, y));
        p.wrapping_add_assign(limb_p);
        let m = n - 1;
        for i in 1..m {
            let y = ys_init[i];
            let (xs_lo, xs_hi) = xs_init.split_at(m - i);
            let limb_p = xs_hi[0].wrapping_mul(y).wrapping_add(
                limbs_slice_add_mul_limb_same_length_in_place_left(&mut out_init[i..], xs_lo, y),
            );
            p.wrapping_add_assign(limb_p);
        }
    }
    *out_last = p;
}

//TODO tune all
const MULLO_BASECASE_THRESHOLD: usize = 0;
const MULLO_DC_THRESHOLD: usize = 62;

const MAYBE_RANGE_BASECASE: bool = TUNE_PROGRAM_BUILD
    || WANT_FAT_BINARY
    || (MULLO_DC_THRESHOLD == 0
        && MULLO_BASECASE_THRESHOLD < MUL_TOOM22_THRESHOLD * 36 / (36 - 11)
        || MULLO_DC_THRESHOLD != 0 && MULLO_DC_THRESHOLD < MUL_TOOM22_THRESHOLD * 36 / (36 - 11));
const MAYBE_RANGE_TOOM22: bool = TUNE_PROGRAM_BUILD
    || WANT_FAT_BINARY
    || (MULLO_DC_THRESHOLD == 0
        && MULLO_BASECASE_THRESHOLD < MUL_TOOM33_THRESHOLD * 36 / (36 - 11)
        || MULLO_DC_THRESHOLD != 0 && MULLO_DC_THRESHOLD < MUL_TOOM33_THRESHOLD * 36 / (36 - 11));

/// This is mpn_dc_mullo_n from mpn/generic/mullo_n.c, where rp == tp.
pub fn _limbs_mul_low_same_length_divide_and_conquer_shared_scratch(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
) {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    assert!(n >= 2);
    // Divide-and-conquer
    // We need fractional approximation of the value 0 < a <= 1/2
    // giving the minimum in the function k=(1-a)^e/(1-2*a^e).
    let n1 = if MAYBE_RANGE_BASECASE && n < MUL_TOOM22_THRESHOLD * 36 / (36 - 11) {
        n >> 1
    } else if MAYBE_RANGE_TOOM22 && n < MUL_TOOM33_THRESHOLD * 36 / (36 - 11) {
        n * 11 / 36 // n1 ~= n*(1-.694...)
    } else if n < MUL_TOOM44_THRESHOLD * 40 / (40 - 9) {
        n * 9 / 40 // n1 ~= n*(1-.775...)
    } else if n < MUL_TOOM8H_THRESHOLD * 10 / 9 {
        n * 7 / 39 // n1 ~= n*(1-.821...)
    } else {
        n / 10 // n1 ~= n*(1-.899...) [TOOM88]
    };
    let n2 = n - n1;
    // Split as x = x1 2^(n2 GMP_NUMB_BITS) + x0, y = y1 2^(n2 GMP_NUMB_BITS) + y0
    // x0 * y0
    limbs_mul_same_length_to_out(out, &xs[..n2], &ys[..n2]);
    // x1 * y0 * 2^(n2 GMP_NUMB_BITS)
    if n1 < MULLO_BASECASE_THRESHOLD {
        _limbs_mul_greater_to_out_basecase(&mut out[n..], &xs[n2..n2 + n1], &ys[..n1]);
    } else if n1 < MULLO_DC_THRESHOLD {
        _limbs_mul_low_same_length_basecase(&mut out[n..], &xs[n2..n2 + n1], &ys[..n1]);
    } else {
        _limbs_mul_low_same_length_divide_and_conquer_shared_scratch(
            &mut out[n..],
            &xs[n2..n2 + n1],
            &ys[..n1],
        );
    }
    let (rp_lo, rp_hi) = out.split_at_mut(n);
    limbs_slice_add_same_length_in_place_left(&mut rp_lo[n2..], &rp_hi[..n1]);
    // x0 * y1 * 2^(n2 GMP_NUMB_BITS)
    if n1 < MULLO_BASECASE_THRESHOLD {
        _limbs_mul_greater_to_out_basecase(&mut out[n..], &xs[..n1], &ys[n2..n2 + n1]);
    } else if n1 < MULLO_DC_THRESHOLD {
        _limbs_mul_low_same_length_basecase(&mut out[n..], &xs[..n1], &ys[n2..n2 + n1]);
    } else {
        _limbs_mul_low_same_length_divide_and_conquer_shared_scratch(
            &mut out[n..],
            &xs[..n1],
            &ys[n2..n2 + n1],
        );
    }
    let (rp_lo, rp_hi) = out.split_at_mut(n);
    limbs_slice_add_same_length_in_place_left(&mut rp_lo[n2..], &rp_hi[..n1]);
}

/// This is mpn_dc_mullo_n from mpn/generic/mullo_n.c, where rp != tp.
pub fn _limbs_mul_low_same_length_divide_and_conquer(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    assert!(n >= 2);
    // Divide-and-conquer
    // We need fractional approximation of the value 0 < a <= 1/2
    // giving the minimum in the function k=(1-a)^e/(1-2*a^e).
    let n1 = if MAYBE_RANGE_BASECASE && n < MUL_TOOM22_THRESHOLD * 36 / (36 - 11) {
        n >> 1
    } else if MAYBE_RANGE_TOOM22 && n < MUL_TOOM33_THRESHOLD * 36 / (36 - 11) {
        n * 11 / 36 // n1 ~= n*(1-.694...)
    } else if n < MUL_TOOM44_THRESHOLD * 40 / (40 - 9) {
        n * 9 / 40 // n1 ~= n*(1-.775...)
    } else if n < MUL_TOOM8H_THRESHOLD * 10 / 9 {
        n * 7 / 39 // n1 ~= n*(1-.821...)
    } else {
        n / 10 // n1 ~= n*(1-.899...) [TOOM88]
    };
    let n2 = n - n1;
    // Split as x = x1 2^(n2 GMP_NUMB_BITS) + x0, y = y1 2^(n2 GMP_NUMB_BITS) + y0
    // x0 * y0
    limbs_mul_same_length_to_out(scratch, &xs[..n2], &ys[..n2]);
    out[..n2].copy_from_slice(&scratch[..n2]);
    // x1 * y0 * 2^(n2 GMP_NUMB_BITS)
    if n1 < MULLO_BASECASE_THRESHOLD {
        _limbs_mul_greater_to_out_basecase(&mut scratch[n..], &xs[n2..n2 + n1], &ys[..n1]);
    } else if n1 < MULLO_DC_THRESHOLD {
        _limbs_mul_low_same_length_basecase(&mut scratch[n..], &xs[n2..n2 + n1], &ys[..n1]);
    } else {
        _limbs_mul_low_same_length_divide_and_conquer_shared_scratch(
            &mut scratch[n..],
            &xs[n2..n2 + n1],
            &ys[..n1],
        );
    }
    limbs_add_same_length_to_out(&mut out[n2..], &scratch[n2..n2 + n1], &scratch[n..n + n1]);
    // x0 * y1 * 2^(n2 GMP_NUMB_BITS)
    if n1 < MULLO_BASECASE_THRESHOLD {
        _limbs_mul_greater_to_out_basecase(&mut scratch[n..], &xs[..n1], &ys[n2..n2 + n1]);
    } else if n1 < MULLO_DC_THRESHOLD {
        _limbs_mul_low_same_length_basecase(&mut scratch[n..], &xs[..n1], &ys[n2..n2 + n1]);
    } else {
        _limbs_mul_low_same_length_divide_and_conquer_shared_scratch(
            &mut scratch[n..],
            &xs[..n1],
            &ys[n2..n2 + n1],
        );
    }
    limbs_slice_add_same_length_in_place_left(&mut out[n2..n2 + n1], &scratch[n..n + n1]);
}

/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// This is mpn_mullo_n_itch from mpn/generic/mullo_n.c.
pub const fn mpn_mullo_n_itch(n: usize) -> usize {
    n << 1
}

/// Time: worst case O(n<sup>2</sup>)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// This is mpn_mullo_basecase from mpn/generic/mullo_basecase.c, MULLO_VARIANT == 1
pub fn _limbs_mul_low_same_length_basecase_alt(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let n = xs.len();
    assert_ne!(n, 0);
    assert_eq!(ys.len(), n);
    let out = &mut out[..n];
    limbs_mul_limb_to_out(out, xs, ys[0]);
    for i in 1..n {
        limbs_slice_add_mul_limb_same_length_in_place_left(&mut out[i..], &xs[..n - i], ys[i]);
    }
}

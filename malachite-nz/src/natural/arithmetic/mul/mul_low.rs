use malachite_base::num::arithmetic::traits::WrappingAddAssign;

use natural::arithmetic::add_mul_limb::limbs_slice_add_mul_limb_same_length_in_place_left;
use natural::arithmetic::mul_limb::limbs_mul_limb_to_out;
use platform::Limb;

/// Time: worst case O(n<sup>2</sup>)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// This is mpn_mullo_basecase from mpn/generic/mullo_basecase.c, MULLO_VARIANT == 2
pub fn _limbs_mul_low_same_length_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
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

/// Time: worst case O(n<sup>2</sup>)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// This is mpn_mullo_basecase from mpn/generic/mullo_basecase.c, MULLO_VARIANT == 1
pub fn _limbs_mul_low_same_length_to_out_alt(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let n = xs.len();
    assert_ne!(n, 0);
    assert_eq!(ys.len(), n);
    let out = &mut out[..n];
    limbs_mul_limb_to_out(out, xs, ys[0]);
    for i in 1..n {
        limbs_slice_add_mul_limb_same_length_in_place_left(&mut out[i..], &xs[..n - i], ys[i]);
    }
}

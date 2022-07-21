use crate::natural::arithmetic::add::limbs_slice_add_greater_in_place_left;
use crate::natural::arithmetic::mul::limbs_mul_greater_to_out_basecase;
use crate::platform::{Limb, MUL_TOOM22_THRESHOLD};

// In GMP this is hardcoded to 500
pub const MUL_BASECASE_MAX_UN: usize = 500;

// T

// We must have 1 < ys.len() < MUL_TOOM22_THRESHOLD < MUL_BASECASE_MAX_UN < xs.len().
fn limbs_mul_greater_to_out_basecase_mem_opt_helper(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(ys_len > 1);
    assert!(ys_len < MUL_TOOM22_THRESHOLD);
    assert!(MUL_TOOM22_THRESHOLD < MUL_BASECASE_MAX_UN);
    assert!(xs_len > MUL_BASECASE_MAX_UN);
    let mut triangle_buffer = [0; MUL_TOOM22_THRESHOLD];
    let mut offset = 0;
    for chunk in xs.chunks(MUL_BASECASE_MAX_UN) {
        let out = &mut out[offset..];
        if chunk.len() >= ys_len {
            limbs_mul_greater_to_out_basecase(out, chunk, ys);
        } else {
            limbs_mul_greater_to_out_basecase(out, ys, chunk);
        }
        if offset != 0 {
            limbs_slice_add_greater_in_place_left(out, &triangle_buffer[..ys_len]);
        }
        offset += MUL_BASECASE_MAX_UN;
        if offset < xs_len {
            triangle_buffer[..ys_len]
                .copy_from_slice(&out[MUL_BASECASE_MAX_UN..MUL_BASECASE_MAX_UN + ys_len]);
        }
    }
}

// T

/// A version of `limbs_mul_greater_to_out_basecase` that attempts to be more efficient by
/// increasing cache locality. It is currently not measurably better than ordinary basecase.
pub fn limbs_mul_greater_to_out_basecase_mem_opt(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    if ys_len > 1 && ys_len < MUL_TOOM22_THRESHOLD && xs.len() > MUL_BASECASE_MAX_UN {
        limbs_mul_greater_to_out_basecase_mem_opt_helper(out, xs, ys)
    } else {
        limbs_mul_greater_to_out_basecase(out, xs, ys);
    }
}

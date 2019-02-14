use malachite_base::num::PrimitiveInteger;
use platform::Limb;
use std::cmp::max;

// This is mpn_toom4_sqr_itch from gmp-impl.h.
fn _limbs_square_to_out_toom_4_scratch_size(xs_len: usize) -> usize {
    3 * xs_len + Limb::WIDTH as usize
}

//TODO tune
const SQR_TOOM6_THRESHOLD: usize = 351;

// This is mpn_toom6_sqr_itch from gmp-impl.h.
pub(crate) fn _limbs_square_to_out_toom_6_scratch_size(n: usize) -> usize {
    let itch = (n as isize - SQR_TOOM6_THRESHOLD as isize) * 2
        + max(
            SQR_TOOM6_THRESHOLD * 2 + Limb::WIDTH as usize * 6,
            _limbs_square_to_out_toom_4_scratch_size(SQR_TOOM6_THRESHOLD),
        ) as isize;
    assert!(itch >= 0);
    itch as usize
}

//TODO tune
const SQR_TOOM8_THRESHOLD: usize = 454;

// This is mpn_toom8_sqr_itch from gmp-impl.h.
pub(crate) fn _limbs_square_to_out_toom_8_scratch_size(n: usize) -> usize {
    let itch = ((n as isize * 15) >> 3) - ((SQR_TOOM8_THRESHOLD as isize * 15) >> 3)
        + max(
            ((SQR_TOOM8_THRESHOLD * 15) >> 3) + Limb::WIDTH as usize * 6,
            _limbs_square_to_out_toom_6_scratch_size(SQR_TOOM8_THRESHOLD),
        ) as isize;
    assert!(itch >= 0);
    itch as usize
}

//TODO PASTE D

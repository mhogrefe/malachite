use std::cmp::max;

use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};

use platform::Limb;

// This is mpn_toom4_sqr_itch from gmp-impl.h.
fn _limbs_square_to_out_toom_4_scratch_len(xs_len: usize) -> usize {
    3 * xs_len + usize::wrapping_from(Limb::WIDTH)
}

//TODO tune
pub(crate) const SQR_TOOM3_THRESHOLD: usize = 93;
const SQR_TOOM6_THRESHOLD: usize = 351;
const SQR_TOOM8_THRESHOLD: usize = 454;

// This is mpn_toom6_sqr_itch from gmp-impl.h.
pub(crate) fn _limbs_square_to_out_toom_6_scratch_len(n: usize) -> usize {
    let itch = (isize::exact_from(n) - isize::exact_from(SQR_TOOM6_THRESHOLD)) * 2
        + isize::exact_from(max(
            SQR_TOOM6_THRESHOLD * 2 + usize::wrapping_from(Limb::WIDTH) * 6,
            _limbs_square_to_out_toom_4_scratch_len(SQR_TOOM6_THRESHOLD),
        ));
    usize::exact_from(itch)
}

// This is mpn_toom8_sqr_itch from gmp-impl.h.
pub(crate) fn _limbs_square_to_out_toom_8_scratch_len(n: usize) -> usize {
    let itch = ((isize::exact_from(n) * 15) >> 3)
        - ((isize::exact_from(SQR_TOOM8_THRESHOLD) * 15) >> 3)
        + isize::exact_from(max(
            ((SQR_TOOM8_THRESHOLD * 15) >> 3) + usize::wrapping_from(Limb::WIDTH) * 6,
            _limbs_square_to_out_toom_6_scratch_len(SQR_TOOM8_THRESHOLD),
        ));
    usize::exact_from(itch)
}

//TODO PASTE D

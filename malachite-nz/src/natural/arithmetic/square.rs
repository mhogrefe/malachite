use std::cmp::max;

use malachite_base::conversion::{CheckedFrom, WrappingFrom};
use malachite_base::num::integers::PrimitiveInteger;

use platform::Limb;

// This is mpn_toom4_sqr_itch from gmp-impl.h.
fn _limbs_square_to_out_toom_4_scratch_size(xs_len: usize) -> usize {
    3 * xs_len + usize::wrapping_from(Limb::WIDTH)
}

//TODO tune
pub(crate) const SQR_TOOM3_THRESHOLD: usize = 93;
const SQR_TOOM6_THRESHOLD: usize = 351;
const SQR_TOOM8_THRESHOLD: usize = 454;

// This is mpn_toom6_sqr_itch from gmp-impl.h.
pub(crate) fn _limbs_square_to_out_toom_6_scratch_size(n: usize) -> usize {
    let itch =
        (isize::checked_from(n).unwrap() - isize::checked_from(SQR_TOOM6_THRESHOLD).unwrap()) * 2
            + isize::checked_from(max(
                SQR_TOOM6_THRESHOLD * 2 + usize::wrapping_from(Limb::WIDTH) * 6,
                _limbs_square_to_out_toom_4_scratch_size(SQR_TOOM6_THRESHOLD),
            ))
            .unwrap();
    usize::checked_from(itch).unwrap()
}

// This is mpn_toom8_sqr_itch from gmp-impl.h.
pub(crate) fn _limbs_square_to_out_toom_8_scratch_size(n: usize) -> usize {
    let itch = ((isize::checked_from(n).unwrap() * 15) >> 3)
        - ((isize::checked_from(SQR_TOOM8_THRESHOLD).unwrap() * 15) >> 3)
        + isize::checked_from(max(
            ((SQR_TOOM8_THRESHOLD * 15) >> 3) + usize::wrapping_from(Limb::WIDTH) * 6,
            _limbs_square_to_out_toom_6_scratch_size(SQR_TOOM8_THRESHOLD),
        ))
        .unwrap();
    usize::checked_from(itch).unwrap()
}

//TODO PASTE D

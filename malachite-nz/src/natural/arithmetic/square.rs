use malachite_base::num::PrimitiveInteger;
use platform::Limb;
use std::cmp::max;

fn _limbs_square_to_out_toom_4_scratch_size(xs_len: usize) -> usize {
    3 * xs_len + Limb::WIDTH as usize
}

//TODO tune
const SQR_TOOM6_THRESHOLD: usize = 351;

pub(crate) fn _limbs_square_to_out_toom_6_scratch_size(n: usize) -> usize {
    let itch = (n as isize - SQR_TOOM6_THRESHOLD as isize) * 2
        + max(
            SQR_TOOM6_THRESHOLD * 2 + Limb::WIDTH as usize * 6,
            _limbs_square_to_out_toom_4_scratch_size(SQR_TOOM6_THRESHOLD),
        ) as isize;
    assert!(itch >= 0);
    itch as usize
}

//TODO PASTE D

use malachite_base::num::arithmetic::traits::Square;
use malachite_base::num::conversion::traits::SplitInHalf;
use malachite_nz::natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use malachite_nz::natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use malachite_nz::natural::arithmetic::square::_limbs_square_diagonal_add_shl_1;
use malachite_nz::platform::{DoubleLimb, Limb};

pub fn _limbs_square_to_out_basecase_unrestricted(out: &mut [Limb], xs: &[Limb]) {
    let n = xs.len();
    let (xs_head, xs_tail) = xs.split_first().unwrap();
    let (square_hi, square_lo) = DoubleLimb::from(*xs_head).square().split_in_half();
    out[0] = square_lo;
    out[1] = square_hi;
    if n > 1 {
        let two_n = n << 1;
        let mut scratch = vec![0; two_n - 2];
        let (scratch_last, scratch_init) = scratch[..n].split_last_mut().unwrap();
        *scratch_last = limbs_mul_limb_to_out(scratch_init, xs_tail, *xs_head);
        for i in 1..n - 1 {
            let (scratch_last, scratch_init) = scratch[i..][i..n].split_last_mut().unwrap();
            let (xs_head, xs_tail) = xs[i..].split_first().unwrap();
            *scratch_last =
                limbs_slice_add_mul_limb_same_length_in_place_left(scratch_init, xs_tail, *xs_head);
        }
        _limbs_square_diagonal_add_shl_1(&mut out[..two_n], &mut scratch, xs);
    }
}

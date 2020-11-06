use malachite_base::num::arithmetic::traits::{Square, WrappingSquare};
use malachite_base::num::conversion::traits::SplitInHalf;
use malachite_nz::natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use malachite_nz::natural::arithmetic::mod_power_of_two_square::_limbs_square_diagonal_shl_add;
use malachite_nz::natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use malachite_nz::platform::{DoubleLimb, Limb};

pub fn _limbs_square_low_basecase_unrestricted(out: &mut [Limb], xs: &[Limb]) {
    let n = xs.len();
    let out = &mut out[..n];
    assert_ne!(n, 0);
    let xs_0 = xs[0];
    match n {
        1 => out[0] = xs_0.wrapping_square(),
        2 => {
            let (p_hi, p_lo) = DoubleLimb::from(xs_0).square().split_in_half();
            out[0] = p_lo;
            out[1] = (xs_0.wrapping_mul(xs[1]) << 1).wrapping_add(p_hi);
        }
        _ => {
            let mut scratch = vec![0; n - 1];
            limbs_mul_limb_to_out(&mut scratch, &xs[1..], xs_0);
            for i in 1.. {
                let two_i = i << 1;
                if two_i >= n - 1 {
                    break;
                }
                limbs_slice_add_mul_limb_same_length_in_place_left(
                    &mut scratch[two_i..],
                    &xs[i + 1..n - i],
                    xs[i],
                );
            }
            _limbs_square_diagonal_shl_add(out, &mut scratch, xs);
        }
    }
}

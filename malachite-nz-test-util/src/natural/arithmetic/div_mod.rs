use malachite_base::num::arithmetic::traits::DivRem;
use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};
use malachite_nz::platform::{DoubleLimb, Limb};

pub fn rug_ceiling_div_neg_mod(x: rug::Integer, y: rug::Integer) -> (rug::Integer, rug::Integer) {
    let (quotient, remainder) = x.div_rem_ceil(y);
    (quotient, -remainder)
}

pub fn limbs_div_limb_to_out_mod_naive(out: &mut [Limb], xs: &[Limb], d: Limb) -> Limb {
    assert!(out.len() >= xs.len());
    let d = DoubleLimb::from(d);
    let mut upper = 0;
    for (out_limb, &in_limb) in out.iter_mut().zip(xs.iter()).rev() {
        let (q, r) = DoubleLimb::join_halves(upper, in_limb).div_rem(d);
        *out_limb = q.lower_half();
        upper = r.lower_half();
    }
    upper
}

pub fn limbs_div_limb_in_place_mod_naive(xs: &mut [Limb], d: Limb) -> Limb {
    let d = DoubleLimb::from(d);
    let mut upper = 0;
    for limb in xs.iter_mut().rev() {
        let (q, r) = DoubleLimb::join_halves(upper, *limb).div_rem(d);
        *limb = q.lower_half();
        upper = r.lower_half();
    }
    upper
}

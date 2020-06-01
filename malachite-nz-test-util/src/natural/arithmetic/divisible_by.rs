use malachite_nz::natural::arithmetic::divisible_by::limbs_divisible_by_limb;
use malachite_nz::natural::arithmetic::mod_op::limbs_mod_limb;
use malachite_nz::platform::{Limb, BMOD_1_TO_MOD_1_THRESHOLD};
use num::{BigUint, Integer, Zero};

pub fn num_divisible_by(x: &BigUint, y: &BigUint) -> bool {
    *x == BigUint::zero() || *y != BigUint::zero() && x.is_multiple_of(y)
}

/// Benchmarks show that this is never faster than just calling `limbs_divisible_by_limb`.
///
/// limbs.len() must be greater than 1; divisor must be nonzero.
///
/// This is mpz_divisible_ui_p from mpz/divis_ui.c, GMP 6.1.2, where a is non-negative.
#[allow(clippy::absurd_extreme_comparisons)]
pub fn combined_limbs_divisible_by_limb(xs: &[Limb], d: Limb) -> bool {
    if xs.len() <= BMOD_1_TO_MOD_1_THRESHOLD {
        limbs_divisible_by_limb(xs, d)
    } else {
        limbs_mod_limb(xs, d) == 0
    }
}

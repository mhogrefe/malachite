use crate::natural::arithmetic::divisible_by::limbs_divisible_by_limb;
use crate::natural::arithmetic::mod_op::limbs_mod_limb;
use num::{BigUint, Integer, Zero};
use crate::platform::{Limb, BMOD_1_TO_MOD_1_THRESHOLD};

pub fn num_divisible_by(x: &BigUint, y: &BigUint) -> bool {
    *x == BigUint::zero() || *y != BigUint::zero() && x.is_multiple_of(y)
}

/// Benchmarks show that this is never faster than just calling `limbs_divisible_by_limb`.
///
/// ns.len() must be greater than 1; divisor must be nonzero.
///
/// This is equivalent to `mpz_divisible_ui_p` from `mpz/divis_ui.c`, GMP 6.2.1, where `a` is
/// non-negative.
#[allow(clippy::absurd_extreme_comparisons)]
pub fn combined_limbs_divisible_by_limb(ns: &[Limb], d: Limb) -> bool {
    if ns.len() <= BMOD_1_TO_MOD_1_THRESHOLD {
        limbs_divisible_by_limb(ns, d)
    } else {
        limbs_mod_limb(ns, d) == 0
    }
}

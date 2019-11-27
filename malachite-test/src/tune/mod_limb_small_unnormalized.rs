use malachite_bench::tune::{compare_two, ComparisonResult};
use malachite_nz::natural::arithmetic::mod_limb::{
    _limbs_mod_limb_small_small, _limbs_mod_limb_small_unnormalized_large,
};
use malachite_nz::platform::Limb;

use common::GenerationMode;
use inputs::base::pairs_of_nonempty_unsigned_vec_and_positive_unsigned_var_1;

pub fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(limbs, divisor): (Vec<Limb>, Limb)| {
            let mut len = limbs.len();
            let mut remainder = limbs[len - 1];
            if remainder < divisor {
                len -= 1;
                if len == 0 {
                    return;
                }
            } else {
                remainder = 0;
            }
            let limbs = &limbs[..len];
            _limbs_mod_limb_small_small(limbs, divisor, remainder);
        }),
        &mut (|(limbs, divisor): (Vec<Limb>, Limb)| {
            let mut len = limbs.len();
            let mut remainder = limbs[len - 1];
            if remainder < divisor {
                len -= 1;
                if len == 0 {
                    return;
                }
            } else {
                remainder = 0;
            }
            let limbs = &limbs[..len];
            _limbs_mod_limb_small_unnormalized_large(limbs, divisor, remainder);
        }),
        pairs_of_nonempty_unsigned_vec_and_positive_unsigned_var_1(GenerationMode::Random(32)),
        10000,
        &(|&(ref limbs, divisor)| {
            if *limbs.last().unwrap() < divisor {
                limbs.len() - 1
            } else {
                limbs.len()
            }
        }),
    );
    let mut lines = Vec::new();
    if let ComparisonResult::SecondBetterAbove(threshold) = result {
        lines.push(format!(
            "pub const MOD_1_UNNORM_THRESHOLD: usize = {};",
            threshold
        ));
    } else {
        panic!(
            "Unexpected mod limb small unnormalized tuning result: {:?}",
            result
        );
    }
    lines
}

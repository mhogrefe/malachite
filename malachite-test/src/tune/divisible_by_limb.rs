use std::cmp::max;

use malachite_bench::tune::{compare_two, ComparisonResult};
use malachite_nz::natural::arithmetic::divisible_by_limb::limbs_divisible_by_limb;
use malachite_nz::natural::arithmetic::mod_op::limbs_mod_limb;
use malachite_nz::platform::Limb;

use common::GenerationMode;
use inputs::base::pairs_of_unsigned_vec_and_positive_unsigned_var_1;

pub fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(limbs, limb): (Vec<Limb>, Limb)| no_out!(limbs_divisible_by_limb(&limbs, limb))),
        &mut (|(limbs, limb): (Vec<Limb>, Limb)| no_out!(limbs_mod_limb(&limbs, limb) == 0)),
        pairs_of_unsigned_vec_and_positive_unsigned_var_1(GenerationMode::Random(512)),
        10000,
        &(|&(ref limbs, _)| limbs.len()),
    );
    let mut lines = Vec::new();
    if let ComparisonResult::SecondBetterAbove(threshold) = result {
        lines.push(format!(
            "pub const BMOD_1_TO_MOD_1_THRESHOLD: usize = {};",
            max(threshold, 1)
        ));
    } else {
        panic!("Unexpected divisible by limb tuning result: {:?}", result);
    }
    lines
}

use malachite_bench::tune::{compare_two, ComparisonResult};
use malachite_nz::natural::arithmetic::mod_op::{
    _limbs_mod_limb_any_leading_zeros, _limbs_mod_limb_small_unnormalized,
};
use malachite_nz::platform::Limb;

use common::GenerationMode;
use inputs::base::pairs_of_unsigned_vec_and_positive_unsigned_var_3;

pub fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(limbs, divisor): (Vec<Limb>, Limb)| {
            no_out!(_limbs_mod_limb_small_unnormalized(&limbs, divisor))
        }),
        &mut (|(limbs, divisor): (Vec<Limb>, Limb)| {
            no_out!(_limbs_mod_limb_any_leading_zeros(&limbs, divisor))
        }),
        pairs_of_unsigned_vec_and_positive_unsigned_var_3(GenerationMode::Random(32)),
        10000,
        &(|&(ref limbs, _)| limbs.len()),
    );
    let mut lines = Vec::new();
    if let ComparisonResult::SecondBetterAbove(threshold) = result {
        lines.push(format!(
            "pub const MOD_1U_TO_MOD_1_1_THRESHOLD: usize = {};",
            threshold
        ));
    } else {
        panic!(
            "Unexpected mod limb any leading zeros from unnormalized tuning result: {:?}",
            result
        );
    }
    lines
}

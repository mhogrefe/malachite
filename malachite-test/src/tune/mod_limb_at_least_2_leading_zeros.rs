use malachite_bench::tune::{compare_two, ComparisonResult};
use malachite_nz::natural::arithmetic::mod_op::{
    _limbs_mod_limb_at_least_1_leading_zero, _limbs_mod_limb_at_least_2_leading_zeros,
};
use malachite_nz::platform::Limb;

use common::GenerationMode;
use inputs::base::pairs_of_nonempty_unsigned_vec_and_positive_unsigned_var_2;

pub(crate) fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(limbs, divisor): (Vec<Limb>, Limb)| {
            no_out!(_limbs_mod_limb_at_least_1_leading_zero(&limbs, divisor))
        }),
        &mut (|(limbs, divisor): (Vec<Limb>, Limb)| {
            no_out!(_limbs_mod_limb_at_least_2_leading_zeros(&limbs, divisor))
        }),
        pairs_of_nonempty_unsigned_vec_and_positive_unsigned_var_2(GenerationMode::Random(32)),
        10000,
        &(|&(ref limbs, _)| limbs.len()),
    );
    let mut lines = Vec::new();
    if let ComparisonResult::SecondBetterAbove(threshold) = result {
        lines.push(format!(
            "pub const MOD_1_2_TO_MOD_1_4_THRESHOLD: usize = {};",
            threshold
        ));
    } else {
        panic!(
            "Unexpected mod limb at least 2 leading zeros tuning result: {:?}",
            result
        );
    }
    lines
}

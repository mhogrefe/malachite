use malachite_bench::tune::{compare_two, ComparisonResult};
use malachite_nz::natural::arithmetic::mul::_limbs_mul_greater_to_out_basecase;
use malachite_nz::natural::arithmetic::mul::mul_low::_limbs_mul_low_same_length_basecase;
use malachite_nz::platform::Limb;

use common::GenerationMode;
use inputs::base::triples_of_unsigned_vec_var_47;

pub fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(mut out, xs, ys): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        }),
        &mut (|(mut out, xs, ys): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            _limbs_mul_low_same_length_basecase(&mut out, &xs, &ys);
        }),
        triples_of_unsigned_vec_var_47(GenerationMode::Random(32)),
        10000,
        &(|&(_, ref xs, _)| xs.len()),
    );
    let mut lines = Vec::new();
    match result {
        ComparisonResult::SecondBetterAbove(threshold) => {
            lines.push(format!(
                "pub const MULLO_BASECASE_THRESHOLD: usize = {};",
                threshold
            ));
        }
        ComparisonResult::SecondAlwaysBetter => {
            lines.push("pub const MULLO_BASECASE_THRESHOLD: usize = 0;".to_string());
        }
        _ => {
            panic!(
                "Unexpected mul basecase to mul low tuning result: {:?}",
                result
            );
        }
    }
    lines
}

use std::hint::black_box;

use malachite_nz::natural::arithmetic::mul::mul_low::{
    _limbs_mul_low_same_length_basecase,
    _limbs_mul_low_same_length_divide_and_conquer_shared_scratch,
};
use malachite_nz::platform::Limb;

use common::GenerationMode;
use inputs::base::triples_of_unsigned_vec_var_48;
use tune::compare_two::{compare_two, ComparisonResult};

pub fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(mut out, xs, ys): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            black_box(_limbs_mul_low_same_length_basecase(&mut out, &xs, &ys))
        }),
        &mut (|(mut out, xs, ys): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            black_box(
                _limbs_mul_low_same_length_divide_and_conquer_shared_scratch(&mut out, &xs, &ys),
            )
        }),
        triples_of_unsigned_vec_var_48(GenerationMode::Random(32)),
        10000,
        &(|&(_, ref xs, _)| xs.len()),
    );
    let mut lines = Vec::new();
    if let ComparisonResult::SecondBetterAbove(threshold) = result {
        lines.push(format!(
            "pub const MULLO_DC_THRESHOLD: usize = {};",
            threshold
        ));
    } else {
        panic!(
            "Unexpected mul low basecase to divide-and-conquer tuning result: {:?}",
            result
        );
    }
    lines
}

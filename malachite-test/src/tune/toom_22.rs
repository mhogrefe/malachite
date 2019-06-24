use malachite_nz::natural::arithmetic::mul::_limbs_mul_greater_to_out_basecase;
use malachite_nz::natural::arithmetic::mul::toom::{
    _limbs_mul_greater_to_out_toom_22, _limbs_mul_greater_to_out_toom_22_scratch_size,
};
use malachite_nz::platform::Limb;

use common::GenerationMode;
use inputs::base::triples_of_unsigned_vec_var_11;
use tune::compare_two::{compare_two, ComparisonResult};

pub fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(mut out, xs, ys): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)
        }),
        &mut (|(mut out, xs, ys): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_22_scratch_size(xs.len())];
            _limbs_mul_greater_to_out_toom_22(&mut out, &xs, &ys, &mut scratch)
        }),
        triples_of_unsigned_vec_var_11(GenerationMode::Random(1024)),
        10000,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
    );
    let mut lines = Vec::new();
    if let ComparisonResult::SecondBetterAbove(threshold) = result {
        lines.push(format!(
            "pub const MUL_TOOM22_THRESHOLD: usize = {};",
            threshold
        ));
    } else {
        panic!("Unexpected Toom22 tuning result: {:?}", result);
    }
    lines
}

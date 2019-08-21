use malachite_nz::natural::arithmetic::mul::toom::{
    _limbs_mul_greater_to_out_toom_32, _limbs_mul_greater_to_out_toom_32_scratch_len,
    _limbs_mul_greater_to_out_toom_43, _limbs_mul_greater_to_out_toom_43_scratch_len,
};
use malachite_nz::platform::Limb;

use common::GenerationMode;
use inputs::base::triples_of_unsigned_vec_var_35;
use tune::compare_two::{compare_two, ComparisonResult};

pub fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(mut out, xs, ys): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            let mut scratch =
                vec![0; _limbs_mul_greater_to_out_toom_32_scratch_len(xs.len(), ys.len())];
            _limbs_mul_greater_to_out_toom_32(&mut out, &xs, &ys, &mut scratch)
        }),
        &mut (|(mut out, xs, ys): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            let mut scratch =
                vec![0; _limbs_mul_greater_to_out_toom_43_scratch_len(xs.len(), ys.len())];
            _limbs_mul_greater_to_out_toom_43(&mut out, &xs, &ys, &mut scratch)
        }),
        triples_of_unsigned_vec_var_35(GenerationMode::Random(1024)),
        10000,
        &(|&(_, _, ref ys)| ys.len()),
    );
    let mut lines = Vec::new();
    if let ComparisonResult::SecondBetterAbove(threshold) = result {
        lines.push(format!(
            "pub const MUL_TOOM32_TO_TOOM43_THRESHOLD: usize = {};",
            threshold
        ));
    } else {
        panic!("Unexpected Toom32 to Toom43 tuning result: {:?}", result);
    }
    lines
}

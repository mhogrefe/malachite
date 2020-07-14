use malachite_bench::tune::{compare_two, ComparisonResult};
use malachite_nz::natural::arithmetic::mul::toom::{
    _limbs_mul_greater_to_out_toom_42, _limbs_mul_greater_to_out_toom_42_scratch_len,
    _limbs_mul_greater_to_out_toom_53, _limbs_mul_greater_to_out_toom_53_scratch_len,
};
use malachite_nz::platform::Limb;

use malachite_test::common::GenerationMode;
use malachite_test::inputs::base::triples_of_unsigned_vec_var_36;

pub(crate) fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(mut out, xs, ys): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            let mut scratch =
                vec![0; _limbs_mul_greater_to_out_toom_42_scratch_len(xs.len(), ys.len())];
            _limbs_mul_greater_to_out_toom_42(&mut out, &xs, &ys, &mut scratch);
        }),
        &mut (|(mut out, xs, ys): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            let mut scratch =
                vec![0; _limbs_mul_greater_to_out_toom_53_scratch_len(xs.len(), ys.len())];
            _limbs_mul_greater_to_out_toom_53(&mut out, &xs, &ys, &mut scratch);
        }),
        triples_of_unsigned_vec_var_36(GenerationMode::Random(1024)),
        10000,
        &(|&(_, _, ref ys)| ys.len()),
    );
    let mut lines = Vec::new();
    if let ComparisonResult::SecondBetterAbove(threshold) = result {
        lines.push(format!(
            "pub const MUL_TOOM42_TO_TOOM53_THRESHOLD: usize = {};",
            threshold
        ));
    } else {
        panic!("Unexpected Toom42 to Toom53 tuning result: {:?}", result);
    }
    lines
}

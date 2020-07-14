use malachite_bench::tune::{compare_two, ComparisonResult};
use malachite_nz::natural::arithmetic::mul::toom::{
    _limbs_mul_greater_to_out_toom_33, _limbs_mul_greater_to_out_toom_33_scratch_len,
    _limbs_mul_greater_to_out_toom_44, _limbs_mul_greater_to_out_toom_44_scratch_len,
};
use malachite_nz::platform::Limb;

use malachite_test::common::GenerationMode;
use malachite_test::inputs::base::triples_of_unsigned_vec_var_31;

pub(crate) fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(mut out, xs, ys): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_33_scratch_len(xs.len())];
            _limbs_mul_greater_to_out_toom_33(&mut out, &xs, &ys, &mut scratch);
        }),
        &mut (|(mut out, xs, ys): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_44_scratch_len(xs.len())];
            _limbs_mul_greater_to_out_toom_44(&mut out, &xs, &ys, &mut scratch);
        }),
        triples_of_unsigned_vec_var_31(GenerationMode::Random(1024)),
        10000,
        &(|&(_, ref xs, _)| xs.len()),
    );
    let mut lines = Vec::new();
    if let ComparisonResult::SecondBetterAbove(threshold) = result {
        lines.push(format!(
            "pub const MUL_TOOM44_THRESHOLD: usize = {};",
            threshold
        ));
    } else {
        panic!("Unexpected Toom44 tuning result: {:?}", result);
    }
    lines
}

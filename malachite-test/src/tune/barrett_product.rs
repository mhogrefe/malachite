use malachite_bench::tune::{compare_two, ComparisonResult};
use malachite_nz::natural::arithmetic::div_mod::_limbs_div_barrett_large_product;
use malachite_nz::natural::arithmetic::mul::limbs_mul_greater_to_out;
use malachite_nz::platform::Limb;

use malachite_test::common::GenerationMode;
use malachite_test::inputs::base::sextuples_of_four_limb_vecs_and_two_usizes_var_1;

pub(crate) fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(mut scratch, ds, qs, _, _, _): (
            Vec<Limb>,
            Vec<Limb>,
            Vec<Limb>,
            Vec<Limb>,
            usize,
            usize,
        )| no_out!(limbs_mul_greater_to_out(&mut scratch, &ds, &qs))),
        &mut (|(mut scratch, ds, qs, rs_hi, scratch_len, i_len): (
            Vec<Limb>,
            Vec<Limb>,
            Vec<Limb>,
            Vec<Limb>,
            usize,
            usize,
        )| {
            _limbs_div_barrett_large_product(&mut scratch, &ds, &qs, &rs_hi, scratch_len, i_len)
        }),
        sextuples_of_four_limb_vecs_and_two_usizes_var_1(GenerationMode::Random(128)),
        10000,
        &(|&(_, _, _, _, _, i_len)| i_len << 1),
    );
    let mut lines = Vec::new();
    if let ComparisonResult::SecondBetterAbove(threshold) = result {
        lines.push(format!(
            "pub const INV_MULMOD_BNM1_THRESHOLD: usize = {};",
            threshold
        ));
    } else {
        panic!("Unexpected Barrett product tuning result: {:?}", result);
    }
    lines
}

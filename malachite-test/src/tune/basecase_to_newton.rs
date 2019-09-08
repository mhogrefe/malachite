use malachite_nz::natural::arithmetic::div_mod::{
    _limbs_invert_basecase_approx, _limbs_invert_newton_approx,
};
use malachite_nz::platform::Limb;

use common::GenerationMode;
use inputs::base::triples_of_unsigned_vec_var_39;
use tune::compare_two::{compare_two, ComparisonResult};

pub fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(mut is, ds, mut scratch): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            _limbs_invert_basecase_approx(&mut is, &ds, &mut scratch);
        }),
        &mut (|(mut is, ds, mut scratch): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            _limbs_invert_newton_approx(&mut is, &ds, &mut scratch);
        }),
        triples_of_unsigned_vec_var_39(GenerationMode::Random(2_048)),
        10000,
        &(|&(_, ref ds, _)| ds.len()),
    );
    let mut lines = Vec::new();
    if let ComparisonResult::SecondBetterAbove(threshold) = result {
        lines.push(format!(
            "pub const INV_NEWTON_THRESHOLD: usize = {};",
            threshold
        ));
    } else {
        panic!("Unexpected basecase to Newton tuning result: {:?}", result);
    }
    lines
}

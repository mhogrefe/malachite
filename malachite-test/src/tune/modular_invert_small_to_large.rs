use malachite_bench::tune::{compare_two, ComparisonResult};
use malachite_nz::natural::arithmetic::div_exact::{
    _limbs_modular_invert_small, limbs_modular_invert,
};
use malachite_nz::platform::Limb;

use common::GenerationMode;
use inputs::base::quadruples_of_three_limb_vecs_and_limb_var_7;

pub(crate) fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(mut is, mut scratch, ds, inverse): (Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)| {
            let n = ds.len();
            _limbs_modular_invert_small(n, &mut is, &mut scratch[..n], &ds, inverse);
        }),
        &mut (|(mut is, mut scratch, ds, _): (Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)| {
            limbs_modular_invert(&mut is, &ds, &mut scratch);
        }),
        quadruples_of_three_limb_vecs_and_limb_var_7(GenerationMode::Random(2_048)),
        10000,
        &(|&(_, _, ref ds, _)| ds.len()),
    );
    let mut lines = Vec::new();
    if let ComparisonResult::SecondBetterAbove(threshold) = result {
        lines.push(format!(
            "pub const BINV_NEWTON_THRESHOLD: usize = {};",
            threshold
        ));
    } else {
        panic!(
            "Unexpected modular invert small to large tuning result: {:?}",
            result
        );
    }
    lines
}

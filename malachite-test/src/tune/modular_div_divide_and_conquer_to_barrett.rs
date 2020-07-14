use malachite_bench::tune::{compare_two, ComparisonResult};
use malachite_nz::natural::arithmetic::div_exact::limbs_modular_invert_limb;
use malachite_nz::natural::arithmetic::div_exact::{
    _limbs_modular_div_barrett, _limbs_modular_div_barrett_scratch_len,
    _limbs_modular_div_divide_and_conquer,
};
use malachite_nz::platform::Limb;

use malachite_test::common::GenerationMode;
use malachite_test::inputs::base::triples_of_limb_vec_var_50;

pub(crate) fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(mut qs, mut ns, ds): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
            _limbs_modular_div_divide_and_conquer(&mut qs, &mut ns, &ds, inverse);
        }),
        &mut (|(mut qs, ns, ds): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            let mut scratch = vec![0; _limbs_modular_div_barrett_scratch_len(ns.len(), ds.len())];
            _limbs_modular_div_barrett(&mut qs, &ns, &ds, &mut scratch);
        }),
        triples_of_limb_vec_var_50(GenerationMode::Random(2_048)),
        10000,
        &(|&(_, ref ns, _)| ns.len()),
    );
    let mut lines = Vec::new();
    if let ComparisonResult::SecondBetterAbove(threshold) = result {
        lines.push(format!(
            "pub const MU_BDIV_Q_THRESHOLD: usize = {};",
            threshold
        ));
    } else {
        panic!(
            "Unexpected modular div divide-and-conquer to Barrett tuning result: {:?}",
            result
        );
    }
    lines
}

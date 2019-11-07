use malachite_nz::natural::arithmetic::div::{
    _limbs_div_barrett_approx, _limbs_div_barrett_approx_scratch_len,
    _limbs_div_divide_and_conquer_approx,
};
use malachite_nz::natural::arithmetic::div_mod::limbs_two_limb_inverse_helper;
use malachite_nz::platform::Limb;

use common::GenerationMode;
use inputs::base::quadruples_of_three_unsigned_vecs_and_unsigned_var_2;
use tune::compare_two::{compare_two, ComparisonResult};

pub fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(mut qs, mut ns, ds, _): (Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)| {
            let inverse = limbs_two_limb_inverse_helper(ds[ds.len() - 1], ds[ds.len() - 2]);
            _limbs_div_divide_and_conquer_approx(&mut qs, &mut ns, &ds, inverse);
        }),
        &mut (|(mut qs, ns, ds, _): (Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)| {
            let mut scratch = vec![0; _limbs_div_barrett_approx_scratch_len(ns.len(), ds.len())];
            _limbs_div_barrett_approx(&mut qs, &ns, &ds, &mut scratch);
        }),
        quadruples_of_three_unsigned_vecs_and_unsigned_var_2(GenerationMode::Random(2_048)),
        10000,
        &(|&(_, ref ns, ref ds, _)| ns.len() - ds.len()),
    );
    let mut lines = Vec::new();
    if let ComparisonResult::SecondBetterAbove(threshold) = result {
        lines.push(format!(
            "pub const MU_DIVAPPR_Q_THRESHOLD: usize = {};",
            threshold
        ));
    } else {
        panic!(
            "Unexpected div approx divide-and-conquer to Barrett tuning result: {:?}",
            result
        );
    }
    lines
}

use std::cmp::min;

use malachite_nz::natural::arithmetic::div_mod::{
    _limbs_div_mod_divide_and_conquer, _limbs_div_mod_schoolbook,
};
use malachite_nz::platform::Limb;

use common::GenerationMode;
use inputs::base::quadruples_of_three_unsigned_vecs_and_unsigned_var_2;
use tune::compare_two::{compare_two, ComparisonResult};

pub fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(mut qs, mut ns, ds, inverse): (Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)| {
            _limbs_div_mod_schoolbook(&mut qs, &mut ns, &ds, inverse);
        }),
        &mut (|(mut qs, mut ns, ds, inverse): (Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)| {
            _limbs_div_mod_divide_and_conquer(&mut qs, &mut ns, &ds, inverse);
        }),
        quadruples_of_three_unsigned_vecs_and_unsigned_var_2(GenerationMode::Random(512)),
        10000,
        &(|&(_, ref ns, ref ds, _)| ns.len() - ds.len()),
    );
    let mut lines = Vec::new();
    if let ComparisonResult::SecondBetterAbove(threshold) = result {
        lines.push(format!(
            "pub const DC_DIV_QR_THRESHOLD: usize = {};",
            min(6, threshold)
        ));
    } else {
        panic!(
            "Unexpected Schoolbook to divide-and-conquer tuning result: {:?}",
            result
        );
    }
    lines
}

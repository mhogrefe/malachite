use malachite_bench::tune::{compare_two, ComparisonResult};
use malachite_nz::natural::arithmetic::div_exact::{
    _limbs_modular_div_divide_and_conquer, _limbs_modular_div_schoolbook,
};
use malachite_nz::platform::Limb;

use common::GenerationMode;
use inputs::base::quadruples_of_three_unsigned_vecs_and_unsigned_var_6;

pub fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(mut qs, mut ns, ds, inverse): (Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)| {
            _limbs_modular_div_schoolbook(&mut qs, &mut ns, &ds, inverse);
        }),
        &mut (|(mut qs, mut ns, ds, inverse): (Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)| {
            _limbs_modular_div_divide_and_conquer(&mut qs, &mut ns, &ds, inverse);
        }),
        quadruples_of_three_unsigned_vecs_and_unsigned_var_6(GenerationMode::Random(2_048)),
        10000,
        &(|&(_, _, ref ds, _)| ds.len()),
    );
    let mut lines = Vec::new();
    if let ComparisonResult::SecondBetterAbove(threshold) = result {
        lines.push(format!(
            "pub const DC_BDIV_Q_THRESHOLD: usize = {};",
            threshold
        ));
    } else {
        panic!(
            "Unexpected modular div schoolbook to divide-and-conquer tuning result: {:?}",
            result
        );
    }
    lines
}

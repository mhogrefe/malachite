use malachite_bench::tune::{compare_two, ComparisonResult};
use malachite_nz::natural::arithmetic::div_exact::{
    _limbs_modular_div_mod_barrett, _limbs_modular_div_mod_barrett_scratch_len,
    _limbs_modular_div_mod_divide_and_conquer,
};
use malachite_nz::natural::arithmetic::div_exact_limb::limbs_modular_invert_limb;
use malachite_nz::platform::Limb;

use common::GenerationMode;
use inputs::base::quadruples_of_unsigned_vec_var_5;

pub fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(mut qs, _, mut ns, ds): (Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
            no_out!(_limbs_modular_div_mod_divide_and_conquer(
                &mut qs, &mut ns, &ds, inverse
            ))
        }),
        &mut (|(mut qs, mut rs, ns, ds): (Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            let mut scratch =
                vec![0; _limbs_modular_div_mod_barrett_scratch_len(ns.len(), ds.len())];
            no_out!(_limbs_modular_div_mod_barrett(
                &mut qs,
                &mut rs,
                &ns,
                &ds,
                &mut scratch
            ))
        }),
        quadruples_of_unsigned_vec_var_5(GenerationMode::Random(2_048)),
        10000,
        &(|&(_, ref ns, _, _)| ns.len()),
    );
    let mut lines = Vec::new();
    match result {
        ComparisonResult::SecondBetterAbove(threshold) => {
            lines.push(format!(
                "pub const MU_BDIV_QR_THRESHOLD: usize = {};",
                threshold
            ));
        }
        ComparisonResult::NeitherBetter => {
            lines.push("pub const MU_BDIV_QR_THRESHOLD: usize = 100000;".to_string());
        }
        _ => {
            panic!(
                "Unexpected modular div/mod divide-and-conquer to Barrett tuning result: {:?}",
                result
            );
        }
    }
    lines
}

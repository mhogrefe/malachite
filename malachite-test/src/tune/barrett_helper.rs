use malachite_bench::tune::{compare_two, ComparisonResult};
use malachite_nz::natural::arithmetic::div_mod::{
    _limbs_div_mod_barrett_helper, _limbs_div_mod_barrett_large_helper,
    _limbs_div_mod_barrett_scratch_len,
};
use malachite_nz::platform::Limb;

use common::GenerationMode;
use inputs::base::quadruples_of_unsigned_vec_var_3;

pub fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(mut qs, mut rs, ns, ds): (Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            let mut scratch = vec![0; _limbs_div_mod_barrett_scratch_len(ns.len(), ds.len())];
            let q_len = ns.len() - ds.len();
            no_out!(_limbs_div_mod_barrett_helper(
                &mut qs[..q_len],
                &mut rs[..ds.len()],
                &ns,
                &ds,
                &mut scratch
            ))
        }),
        &mut (|(mut qs, mut rs, ns, ds): (Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            let mut scratch = vec![0; _limbs_div_mod_barrett_scratch_len(ns.len(), ds.len())];
            let q_len = ns.len() - ds.len();
            no_out!(_limbs_div_mod_barrett_large_helper(
                &mut qs[..q_len],
                &mut rs[..ds.len()],
                &ns,
                &ds,
                &mut scratch
            ))
        }),
        quadruples_of_unsigned_vec_var_3(GenerationMode::Random(2_048)),
        10000,
        &(|&(_, _, ref ns, ref ds)| (ds.len() << 1).saturating_sub(ns.len())),
    );
    let mut lines = Vec::new();
    if let ComparisonResult::SecondBetterAbove(threshold) = result {
        lines.push(format!(
            "pub const MU_DIV_QR_SKEW_THRESHOLD: usize = {};",
            threshold
        ));
    } else {
        panic!("Unexpected Barrett helper tuning result: {:?}", result);
    }
    lines
}

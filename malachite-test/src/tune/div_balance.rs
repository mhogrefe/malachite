use std::cmp::max;

use malachite_bench::tune::{compare_two, ComparisonResult};
use malachite_nz::natural::arithmetic::div::{
    _limbs_div_to_out_balanced, _limbs_div_to_out_unbalanced,
};
use malachite_nz::platform::Limb;

use common::GenerationMode;
use inputs::base::triples_of_unsigned_vec_var_44;

pub(crate) fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(mut qs, mut ns, mut ds): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            _limbs_div_to_out_unbalanced(&mut qs, &mut ns, &mut ds);
        }),
        &mut (|(mut qs, ns, ds): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            _limbs_div_to_out_balanced(&mut qs, &ns, &ds);
        }),
        triples_of_unsigned_vec_var_44(GenerationMode::Random(512)),
        10000,
        &(|&(_, ref ns, ref ds)| max(2, (ds.len() << 1).saturating_sub(ns.len()))),
    );
    let mut lines = Vec::new();
    if let ComparisonResult::SecondBetterAbove(threshold) = result {
        lines.push(format!("pub const FUDGE: usize = {};", threshold));
    } else {
        panic!("Unexpected div balance tuning result: {:?}", result);
    }
    lines
}

use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::logic::traits::BitAccess;
use malachite_bench::tune::{compare_two, ComparisonResult};
use malachite_nz::natural::arithmetic::div_mod::{
    _limbs_div_mod_barrett, _limbs_div_mod_barrett_scratch_len, _limbs_div_mod_divide_and_conquer,
    limbs_two_limb_inverse_helper,
};
use malachite_nz::platform::Limb;

use common::GenerationMode;
use inputs::base::triples_of_limb_vec_var_40;

pub(crate) fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(mut qs, mut ns, mut ds): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            let q_len = ns.len() - ds.len() + 1;
            ds[q_len - 1].set_bit(u64::from(Limb::WIDTH) - 1);
            let inverse = limbs_two_limb_inverse_helper(ds[q_len - 1], ds[q_len - 2]);
            no_out!(_limbs_div_mod_divide_and_conquer(
                &mut qs,
                &mut ns[..q_len << 1],
                &ds[..q_len],
                inverse
            ))
        }),
        &mut (|(mut qs, mut ns, mut ds): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            let d_len = ds.len();
            let mut rs = vec![0; d_len];
            let q_len = ns.len() - d_len + 1;
            let q_len_2 = q_len << 1;
            ds[q_len - 1].set_bit(u64::from(Limb::WIDTH) - 1);
            limbs_two_limb_inverse_helper(ds[q_len - 1], ds[q_len - 2]);
            let mut scratch = vec![0; _limbs_div_mod_barrett_scratch_len(q_len_2, q_len)];
            _limbs_div_mod_barrett(&mut qs, &mut rs, &ns[..q_len_2], &ds[..q_len], &mut scratch);
            ns[..q_len].copy_from_slice(&rs[..q_len]);
        }),
        triples_of_limb_vec_var_40(GenerationMode::Random(4_096)),
        10000,
        &(|&(_, ref ns, ref ds)| ns.len() - ds.len()),
    );
    let mut lines = Vec::new();
    if let ComparisonResult::SecondBetterAbove(threshold) = result {
        lines.push(format!(
            "pub const MU_DIV_QR_THRESHOLD: usize = {};",
            threshold
        ));
    } else {
        panic!(
            "Unexpected div/mod divide-and-conquer to Barrett tuning result: {:?}",
            result
        );
    }
    lines
}

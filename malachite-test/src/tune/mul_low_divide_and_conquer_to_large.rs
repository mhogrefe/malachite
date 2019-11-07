use std::hint::black_box;

use malachite_nz::natural::arithmetic::mul::mul_low::{
    _limbs_mul_low_same_length_divide_and_conquer,
    _limbs_mul_low_same_length_divide_and_conquer_scratch_len, _limbs_mul_low_same_length_large,
};
use malachite_nz::platform::Limb;

use common::GenerationMode;
use inputs::base::triples_of_unsigned_vec_var_52;
use tune::compare_two::{compare_two, ComparisonResult};

pub fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(mut out, xs, ys): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            let mut scratch =
                vec![0; _limbs_mul_low_same_length_divide_and_conquer_scratch_len(xs.len())];
            black_box(_limbs_mul_low_same_length_divide_and_conquer(
                &mut out,
                &xs,
                &ys,
                &mut scratch,
            ))
        }),
        &mut (|(mut out, xs, ys): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            let mut scratch =
                vec![0; _limbs_mul_low_same_length_divide_and_conquer_scratch_len(xs.len())];
            black_box(_limbs_mul_low_same_length_large(
                &mut out,
                &xs,
                &ys,
                &mut scratch,
            ))
        }),
        triples_of_unsigned_vec_var_52(GenerationMode::Random(1 << 15)),
        10000,
        &(|&(_, ref xs, _)| xs.len()),
    );
    let mut lines = Vec::new();
    match result {
        ComparisonResult::SecondBetterAbove(threshold) => {
            lines.push(format!(
                "pub const MULLO_MUL_N_THRESHOLD: usize = {};",
                threshold
            ));
        }
        ComparisonResult::NeitherBetter => {
            lines.push("pub const MULLO_MUL_N_THRESHOLD: usize = 100000;".to_string());
        }
        _ => {
            panic!(
                "Unexpected mul low divide-and-conquer to large tuning result: {:?}",
                result
            );
        }
    }
    lines
}

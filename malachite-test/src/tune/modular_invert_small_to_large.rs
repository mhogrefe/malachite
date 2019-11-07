use std::hint::black_box;

use malachite_nz::natural::arithmetic::div_exact::{
    _limbs_modular_invert_small, limbs_modular_invert,
};
use malachite_nz::platform::Limb;

use common::GenerationMode;
use inputs::base::quadruples_of_three_unsigned_vecs_and_unsigned_var_7;
use tune::compare_two::{compare_two, ComparisonResult};

pub fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(mut is, mut scratch, ds, inverse): (Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)| {
            let n = ds.len();
            black_box(_limbs_modular_invert_small(
                n,
                &mut is,
                &mut scratch[..n],
                &ds,
                inverse,
            ));
        }),
        &mut (|(mut is, mut scratch, ds, _): (Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)| {
            black_box(limbs_modular_invert(&mut is, &ds, &mut scratch));
        }),
        quadruples_of_three_unsigned_vecs_and_unsigned_var_7(GenerationMode::Random(2_048)),
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

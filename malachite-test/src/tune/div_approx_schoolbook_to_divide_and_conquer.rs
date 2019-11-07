use malachite_nz::natural::arithmetic::div::{
    _limbs_div_divide_and_conquer_approx, _limbs_div_schoolbook_approx,
};
use malachite_nz::platform::{Limb, DC_DIVAPPR_Q_THRESHOLD};

use common::GenerationMode;
use inputs::base::quadruples_of_three_unsigned_vecs_and_unsigned_var_2;
use tune::compare_two::{compare_two, ComparisonResult};

pub fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(mut qs, mut ns, ds, inverse): (Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)| {
            _limbs_div_schoolbook_approx(&mut qs, &mut ns, &ds, inverse);
        }),
        &mut (|(mut qs, mut ns, ds, inverse): (Vec<Limb>, Vec<Limb>, Vec<Limb>, Limb)| {
            _limbs_div_divide_and_conquer_approx(&mut qs, &mut ns, &ds, inverse);
        }),
        quadruples_of_three_unsigned_vecs_and_unsigned_var_2(GenerationMode::Random(1_024)),
        10000,
        &(|&(_, _, ref ds, _)| ds.len()),
    );
    let mut lines = Vec::new();
    match result {
        ComparisonResult::SecondBetterAbove(threshold) => {
            lines.push(format!(
                "pub const DC_DIVAPPR_Q_THRESHOLD: usize = {};",
                threshold
            ));
            lines.push("pub const MAYBE_DCP1_DIVAPPR: bool = true;".to_string());
        }
        ComparisonResult::FirstAlwaysBetter => {
            // keep old value
            lines.push(format!(
                "pub const DC_DIVAPPR_Q_THRESHOLD: usize = {};",
                DC_DIVAPPR_Q_THRESHOLD
            ));
            lines.push("pub const MAYBE_DCP1_DIVAPPR: bool = false;".to_string());
        }
        _ => panic!(
            "Unexpected div approx schoolbook to divide-and-conquer tuning result: {:?}",
            result
        ),
    }
    lines
}

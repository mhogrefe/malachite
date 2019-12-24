use malachite_bench::tune::{compare_two, ComparisonResult};
use malachite_nz::natural::arithmetic::mod_op::{
    _limbs_mod_limb_any_leading_zeros_1, _limbs_mod_limb_any_leading_zeros_2,
};
use malachite_nz::platform::Limb;

use common::GenerationMode;
use inputs::base::pairs_of_unsigned_vec_and_positive_unsigned_var_1;

pub(crate) fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(limbs, divisor): (Vec<Limb>, Limb)| {
            no_out!(_limbs_mod_limb_any_leading_zeros_1(&limbs, divisor))
        }),
        &mut (|(limbs, divisor): (Vec<Limb>, Limb)| {
            no_out!(_limbs_mod_limb_any_leading_zeros_2(&limbs, divisor))
        }),
        pairs_of_unsigned_vec_and_positive_unsigned_var_1(GenerationMode::Random(512)),
        10000,
        &(|&(ref limbs, _)| limbs.len()),
    );
    let mut lines = Vec::new();
    if result == ComparisonResult::FirstAlwaysBetter {
        lines.push("pub const MOD_1_1P_METHOD: bool = true;".to_string());
    } else if result == ComparisonResult::SecondAlwaysBetter {
        lines.push("pub const MOD_1_1P_METHOD: bool = false;".to_string());
    } else {
        panic!(
            "Unexpected mod limb any leading zeros tuning result: {:?}",
            result
        );
    }
    lines
}
